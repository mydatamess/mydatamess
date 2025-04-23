import * as React from "react";
import { styled, useTheme } from "@mui/material/styles";
import MuiDrawer, { drawerClasses } from "@mui/material/Drawer";
import Box from "@mui/material/Box";
import Divider from "@mui/material/Divider";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import SettingsRoundedIcon from "@mui/icons-material/SettingsRounded";
import MenuContent from "./MenuContent";
import useMediaQuery from "@mui/material/useMediaQuery";
import { useTranslation } from "react-i18next";

const drawerWidth = 240;

const Drawer = styled(MuiDrawer)({
  width: drawerWidth,
  flexShrink: 0,
  boxSizing: "border-box",
  [`& .${drawerClasses.paper}`]: {
    width: drawerWidth,
    boxSizing: "border-box",
  },
});

interface SideMenuProps {
  mobileMenuOpen: boolean;
  toggleMobileMenu: (open: boolean) => () => void;
}

export default function SideMenu({
  mobileMenuOpen,
  toggleMobileMenu,
}: SideMenuProps): React.JSX.Element {
  const { t } = useTranslation(["common"]);
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("md"));

  return (
    <Drawer
      anchor="left"
      variant={isMobile ? "temporary" : "permanent"}
      open={isMobile ? mobileMenuOpen : true}
      onClose={isMobile ? toggleMobileMenu(false) : undefined}
      sx={{
        display: { xs: "block", md: "block" },
        zIndex: (theme) => theme.zIndex.drawer + 1,
        [`& .${drawerClasses.paper}`]: {
          backgroundColor: "background.paper",
        },
      }}
    >
      <Box
        sx={{
          display: "flex",
          mt: "calc(var(--template-frame-height, 0px) + 4px)",
          p: 1.5,
          justifyContent: "center",
        }}
      >
        <Typography variant="subtitle1">
          my<strong>data</strong>mess
        </Typography>
      </Box>
      <Divider />
      <Box
        sx={{
          overflow: "auto",
          height: "100%",
          display: "flex",
          flexDirection: "column",
        }}
      >
        <MenuContent />
      </Box>
      <Stack
        direction="row"
        sx={{
          p: 2,
          gap: 1,
          alignItems: "center",
          borderTop: "1px solid",
          borderColor: "divider",
        }}
      >
        <Button
          component="label"
          role={undefined}
          variant="outlined"
          tabIndex={-1}
          startIcon={<SettingsRoundedIcon color="action" />}
          size="small"
          color="primary"
          fullWidth
        >
          {t("common:menu.settings")}
        </Button>
      </Stack>
    </Drawer>
  );
}

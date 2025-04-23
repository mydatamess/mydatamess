import React, { useState } from "react";
import Grid from "@mui/material/Grid2";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import ResourceCard from "./ResourceCard";
import { useAppContext } from "../context/AppContext";
import { useAsync } from "react-use";

export default function MainGrid(): React.JSX.Element {
  const {
    services: { resourceService },
  } = useAppContext();
  const [resources, setResources] = useState<string[]>([]);

  useAsync(async () => {
    const rootRes = await resourceService.getRootResource();
    console.log(rootRes);

    if (rootRes.ok) {
      setResources([rootRes.value.resource.displayName]);
    }
  }, [resourceService, setResources]);

  return (
    <Box sx={{ width: "100%", maxWidth: { sm: "100%", md: "1700px" } }}>
      {/* cards */}
      <Typography component="h2" variant="h6" sx={{ mb: 2 }}>
        Overview
      </Typography>
      <Grid
        container
        spacing={2}
        columns={12}
        sx={{ mb: (theme) => theme.spacing(2) }}
      >
        {resources.map((resource) => (
          <Grid key={resource} size={{ xs: 12, sm: 6, lg: 3 }}>
            <ResourceCard
              key={resource}
              title={resource}
              description={`A description for ${resource}`}
            />
          </Grid>
        ))}
      </Grid>
      <Typography component="h2" variant="h6" sx={{ mb: 2 }}>
        Details
      </Typography>
    </Box>
  );
}

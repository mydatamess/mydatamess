import React from "react";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";

export default function ResourceCard({
  title,
  description,
}: {
  title: string;
  description?: string;
}): React.JSX.Element {
  return (
    <Card sx={{ height: "100%" }}>
      <CardContent>
        <Typography
          component="h2"
          variant="subtitle2"
          gutterBottom
          sx={{ fontWeight: "600" }}
        >
          {title}
        </Typography>
        {description && (
          <Typography sx={{ color: "text.secondary", mb: "8px" }}>
            {description}
          </Typography>
        )}
      </CardContent>
    </Card>
  );
}

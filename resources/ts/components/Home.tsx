import React from "react";
import { Route, Router, Switch } from "wouter";

import { ChangeModelPage } from "./ChangeModelPage";
import { DashboardPage } from "./DashboardPage";
import { PromptContextProvider } from "./PromptContextProvider";
import { PromptPage } from "./PromptPage";
import { WorkbenchLayout } from "./WorkbenchLayout";

export function Home({
  bufferedRequestTimeoutMillis,
  inferenceAddr,
  managementAddr,
  maxBufferedRequests,
}: {
  bufferedRequestTimeoutMillis: number;
  inferenceAddr: string;
  managementAddr: string;
  maxBufferedRequests: number;
}) {
  return (
    <Router>
      <WorkbenchLayout>
        <Switch>
          <Route path="/">
            <DashboardPage
              bufferedRequestTimeoutMillis={bufferedRequestTimeoutMillis}
              inferenceAddr={inferenceAddr}
              managementAddr={managementAddr}
              maxBufferedRequests={maxBufferedRequests}
            />
          </Route>
          <Route path="/model">
            <ChangeModelPage managementAddr={managementAddr} />
          </Route>
          <Route path="/prompt">
            <PromptContextProvider>
              <PromptPage inferenceAddr={inferenceAddr} />
            </PromptContextProvider>
          </Route>
          <Route>404 :(</Route>
        </Switch>
      </WorkbenchLayout>
    </Router>
  );
}

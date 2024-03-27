import { BrowserRouter, Route, Routes } from "react-router-dom";

import Player from "./pages/player.tsx";
import { ApiServices } from "./services/api.ts";
import Playlists from "./pages/playlists.tsx";

interface Props {
  apiServices: ApiServices;
}

function App(props: Props) {
  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<Playlists apiServices={props.apiServices} />}
        ></Route>
        <Route
          path="/playlists"
          element={<Playlists apiServices={props.apiServices} />}
        ></Route>
        <Route
          path="/player"
          element={<Player apiServices={props.apiServices} />}
        ></Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;

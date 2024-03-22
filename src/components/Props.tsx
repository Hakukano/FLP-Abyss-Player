import { EntryBrief, EntryDetails } from "../services/api/entry";
import { GroupBrief } from "../services/api/group";
import { ApiServices } from "../services/api";

export interface Props {
  apiServices: ApiServices;
  playlist: Playlist;
  groups: GroupBrief[];
  entries: EntryBrief[];
  entry: EntryDetails;
}

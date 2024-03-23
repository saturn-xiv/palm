import { useState, useEffect } from "react";
import { styled } from "@mui/material/styles";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select, { SelectChangeEvent } from "@mui/material/Select";
import Button from "@mui/material/Button";
import CloudUploadIcon from "@mui/icons-material/CloudUpload";
import { FormattedMessage } from "react-intl";

import { IAttachment, index_picture } from "../../api/camelia";

interface IProps {
  form: string;
  id: number | null;
  label: string;
  handleChange: (id: number) => void;
}

const Widget = ({ form, id, label, handleChange }: IProps) => {
  const [items, setItems] = useState<IAttachment[]>([]);
  const label_id = `${form}-attachment-select-label`;
  useEffect(() => {
    index_picture().then((res) => {
      setItems(res);
    });
  }, []);
  return (
    <>
      <FormControl margin="normal" required fullWidth>
        <InputLabel id={label_id}>{label}</InputLabel>
        <Select
          labelId={label_id}
          value={id || ""}
          onChange={(e: SelectChangeEvent<number>) => {
            handleChange(e.target.value as number);
          }}
        >
          {items.map((x, i) => (
            <MenuItem key={i} value={x.id}>
              {x.title}
            </MenuItem>
          ))}
        </Select>
      </FormControl>

      <FormControl margin="normal">
        <Button
          component="label"
          variant="contained"
          startIcon={<CloudUploadIcon />}
        >
          <FormattedMessage id="buttons.upload" />
          <input
            onChange={({ target }) => {
              if (target.files) {
                console.log(`upload file ${target.files[0].name}`);
                // const fileReader = new FileReader();
                // const name = target.accept;

                // fileReader.readAsDataURL(target.files[0]);
                // fileReader.onload = (e) => {
                //   this.setState((prevState) => ({
                //     [name]: [...prevState[name], e.target.result],
                //   }));
                // };
              }
            }}
            accept="image/*"
            type="file"
            hidden
          />
        </Button>
      </FormControl>
    </>
  );
};

export default Widget;

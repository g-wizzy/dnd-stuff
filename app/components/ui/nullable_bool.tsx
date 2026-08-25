import { SegmentedControl } from "@mantine/core";

interface NullableBoolProps {
  textForTrue: string;
  textForNull: string;
  textForFalse: string;
  value: boolean | null;
  setValue: (value: boolean | null) => void;
}

export default function NullableBool({ textForTrue, textForNull, textForFalse, value, setValue }: NullableBoolProps) {
  const setValueFromControl: (newValue: string) => void = (newValue) => {
    setValue(
      newValue === textForTrue ? true :
        newValue === textForFalse ? false :
          null
    )
  };
  const textValue = value === true ? textForTrue :
    value === false ? textForFalse :
      textForNull;
  return <SegmentedControl
    data={[textForFalse, textForNull, textForTrue]}
    value={textValue}
    onChange={setValueFromControl}
  />;
}

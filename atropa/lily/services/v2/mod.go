package v2

import (
	"fmt"
)

func (p *TeXLiveTask_Output_Format) File(name string) (string, error) {
	switch *p {
	case TeXLiveTask_Output_Pdf:
		return name + ".pdf", nil
	default:
		return "", fmt.Errorf("unsupported ext %s", p.String())
	}
}

func (p *PandocTask_Format) File(name string) (string, error) {
	switch *p {
	case PandocTask_Pdf:
		return name + ".pdf", nil
	case PandocTask_Docx:
		return name + ".docx", nil
	case PandocTask_Plain:
		return name + ".tex", nil
	case PandocTask_Markdown:
		return name + ".md", nil
	default:
		return "", fmt.Errorf("unsupported ext %s", p.String())
	}
}

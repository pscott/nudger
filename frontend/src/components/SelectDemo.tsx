import { useDemoStore } from "@/store/DemoStore";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export default function SelectDemo() {
  const { selectedDemo, setSelectedDemo } = useDemoStore();

  return (
    <Select value={selectedDemo} onValueChange={setSelectedDemo}>
      <SelectTrigger className="w-[180px] bg-[#b3e5fc] text-[#1565c0] rounded-full border-none focus:ring-0 focus:ring-offset-0 pr-3 font-medium hover:bg-[#81d4fa] transition-colors">
        <SelectValue placeholder="Navbar" />
      </SelectTrigger>
      <SelectContent className="bg-[#b3e5fc] border-none rounded-xl shadow-lg">
        <SelectItem
          className="text-[#1565c0] hover:bg-[#81d4fa] hover:text-[#0d47a1] focus:bg-[#81d4fa] focus:text-[#0d47a1] rounded-lg transition-colors hover:font-semibold"
          value="navbar"
        >
          Navbar
        </SelectItem>
        <SelectItem
          className="text-[#1565c0] hover:bg-[#81d4fa] hover:text-[#0d47a1] focus:bg-[#81d4fa] focus:text-[#0d47a1] rounded-lg transition-colors hover:font-semibold"
          value="hover-card"
        >
          Hover Card
        </SelectItem>
        <SelectItem
          className="text-[#1565c0] hover:bg-[#81d4fa] hover:text-[#0d47a1] focus:bg-[#81d4fa] focus:text-[#0d47a1] rounded-lg transition-colors hover:font-semibold"
          value="toaster"
        >
          Toaster
        </SelectItem>
      </SelectContent>
    </Select>
  );
}

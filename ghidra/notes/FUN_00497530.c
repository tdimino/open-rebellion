
int __fastcall FUN_00497530(int param_1)

{
  if (*(int *)(param_1 + 0x20) == 1) {
    return (-(uint)(*(int *)(param_1 + 0x6c) != 1) & 0xfffffffc) + 7;
  }
  return (-(uint)(*(int *)(param_1 + 0x6c) != 2) & 0xfffffffc) + 7;
}



void __fastcall FUN_004317b0(int param_1)

{
  *(undefined4 *)(param_1 + 0x30) = 0;
  *(undefined4 *)(param_1 + 0x34) = 0;
  *(uint *)(param_1 + 0x2c) = *(uint *)(param_1 + 0x2c) & 0xdfffffff;
  FUN_004321d0(param_1);
  return;
}


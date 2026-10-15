
void __thiscall FUN_004c3010(int param_1,uint param_2)

{
  if ((param_2 & 0x100000) != 0) {
    *(int *)(*(int *)(param_1 + 0x168) + 0x15c) = DAT_006b28cc + 40000;
    FUN_0041d810();
    return;
  }
  if ((param_2 & 0x1000000) != 0) {
    *(int *)(*(int *)(param_1 + 0x168) + 0x158) = DAT_006b28cc + 40000;
    FUN_0041d810();
  }
  return;
}


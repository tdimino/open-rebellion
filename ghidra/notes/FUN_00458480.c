
void __fastcall FUN_00458480(int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  
  iVar2 = 3;
  puVar1 = (undefined4 *)(param_1 + 0x1c0);
  do {
    puVar1[-1] = 0;
    *puVar1 = 0;
    puVar1[1] = 0;
    puVar1[3] = 0;
    puVar1[2] = 0;
    puVar1[5] = 0;
    puVar1[4] = 0;
    puVar1 = puVar1 + 0xe;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  SetRect((LPRECT)(param_1 + 0x1a0),0x37,0x39,0xdd,0x88);
  SetRect((LPRECT)(param_1 + 0x1d8),0x37,0x8a,0xdd,0xd9);
  SetRect((LPRECT)(param_1 + 0x210),0x37,0xdb,0xdd,0x12a);
  return;
}


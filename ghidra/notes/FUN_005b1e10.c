
void __cdecl FUN_005b1e10(int param_1)

{
  uint *puVar1;
  void *this;
  int iVar2;
  uint uVar3;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar3 = *puVar1;
  this = (void *)FUN_00596bd0();
  iVar2 = FUN_005a6340(this,uVar3);
  if (iVar2 != 0) {
    *(undefined4 *)(iVar2 + 0x35c) = 2;
    *(uint *)(iVar2 + 0x164) = puVar1[1];
  }
  return;
}



void __cdecl FUN_005caf20(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  uint uVar3;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar3 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  pvVar2 = (void *)FUN_005a6480(pvVar2,uVar3);
  if (pvVar2 != (void *)0x0) {
    FUN_005caf50(pvVar2,puVar1[1]);
  }
  return;
}


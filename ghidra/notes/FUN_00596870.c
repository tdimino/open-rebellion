
void __cdecl FUN_00596870(undefined4 *param_1,uint param_2)

{
  void *pvVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined4 *puVar4;
  undefined4 local_2c [11];
  
  pvVar1 = (void *)FUN_00596bd0();
  pvVar1 = (void *)FUN_005a6340(pvVar1,param_2);
  if (pvVar1 == (void *)0x0) {
    iVar3 = FUN_00596bd0();
    pvVar1 = (void *)FUN_005a66a0(iVar3);
    if (pvVar1 == (void *)0x0) goto LAB_005968c1;
    puVar2 = (undefined4 *)FUN_005ba670(pvVar1,local_2c);
  }
  else {
    puVar2 = (undefined4 *)FUN_005b0280(pvVar1,local_2c);
  }
  puVar4 = local_2c;
  for (iVar3 = 0xb; iVar3 != 0; iVar3 = iVar3 + -1) {
    *puVar4 = *puVar2;
    puVar2 = puVar2 + 1;
    puVar4 = puVar4 + 1;
  }
LAB_005968c1:
  puVar2 = local_2c;
  for (iVar3 = 0xb; iVar3 != 0; iVar3 = iVar3 + -1) {
    *param_1 = *puVar2;
    puVar2 = puVar2 + 1;
    param_1 = param_1 + 1;
  }
  return;
}



void __cdecl FUN_005968e0(undefined4 *param_1,uint param_2)

{
  void *pvVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined4 *puVar4;
  undefined4 local_20 [8];
  
  pvVar1 = (void *)FUN_00596bd0();
  pvVar1 = (void *)FUN_005a6340(pvVar1,param_2);
  if (pvVar1 != (void *)0x0) {
    puVar2 = (undefined4 *)FUN_005b8e00(pvVar1,local_20);
    puVar4 = local_20;
    for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
      *puVar4 = *puVar2;
      puVar2 = puVar2 + 1;
      puVar4 = puVar4 + 1;
    }
  }
  puVar2 = local_20;
  for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
    *param_1 = *puVar2;
    puVar2 = puVar2 + 1;
    param_1 = param_1 + 1;
  }
  return;
}


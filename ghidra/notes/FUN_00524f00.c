
bool FUN_00524f00(uint *param_1,uint *param_2,void *param_3)

{
  uint *puVar1;
  void *pvVar2;
  int iVar3;
  bool bVar4;
  
  bVar4 = true;
  puVar1 = FUN_004ece40(param_1);
  if (puVar1 != (uint *)0x0) {
    pvVar2 = (void *)FUN_00504dc0(param_1);
    iVar3 = FUN_004f8090(pvVar2,param_3);
    bVar4 = iVar3 != 0;
  }
  puVar1 = FUN_004ece40(param_2);
  if (puVar1 != (uint *)0x0) {
    pvVar2 = (void *)FUN_00504dc0(param_2);
    iVar3 = FUN_004f8040(pvVar2,param_3);
    if ((iVar3 != 0) && (bVar4)) {
      return true;
    }
    bVar4 = false;
  }
  return bVar4;
}


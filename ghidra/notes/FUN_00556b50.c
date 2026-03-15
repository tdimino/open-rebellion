
undefined4 FUN_00556b50(void *param_1)

{
  void *pvVar1;
  void *pvVar2;
  undefined4 uVar3;
  int iVar4;
  void *this;
  int iVar5;
  
  iVar4 = 0;
  for (pvVar1 = (void *)FUN_00505ef0(); pvVar1 != (void *)0x0; pvVar1 = (void *)FUN_0051a5d0(pvVar1)
      ) {
    if ((*(byte *)((int)pvVar1 + 0x50) & 0x40) != 0) {
      iVar4 = iVar4 + 1;
    }
  }
  if (iVar4 == 0) {
    return 1;
  }
  iVar4 = FUN_0053e290(iVar4 + -1);
  iVar5 = 0;
  pvVar1 = (void *)FUN_00505ef0();
  this = (void *)0x0;
  while (pvVar2 = pvVar1, pvVar2 != (void *)0x0) {
    if (this != (void *)0x0) goto LAB_00556bbc;
    if ((*(byte *)((int)pvVar2 + 0x50) & 0x40) == 0) {
LAB_00556baf:
      pvVar1 = (void *)FUN_0051a5d0(pvVar2);
    }
    else {
      if (iVar4 != iVar5) {
        iVar5 = iVar5 + 1;
        goto LAB_00556baf;
      }
      pvVar1 = (void *)FUN_0051a5d0(pvVar2);
      this = pvVar2;
    }
  }
  if (this == (void *)0x0) {
    return 1;
  }
LAB_00556bbc:
  uVar3 = FUN_0050cdc0(this,param_1);
  return uVar3;
}


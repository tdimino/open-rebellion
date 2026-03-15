
int FUN_00556be0(void *param_1)

{
  void *pvVar1;
  void *pvVar2;
  int iVar3;
  void *this;
  int iVar4;
  
  iVar3 = 0;
  for (pvVar1 = (void *)FUN_00505ef0(); pvVar1 != (void *)0x0; pvVar1 = (void *)FUN_0051a5d0(pvVar1)
      ) {
    if ((*(byte *)((int)pvVar1 + 0x50) & 0x40) != 0) {
      iVar3 = iVar3 + 1;
    }
  }
  if (iVar3 == 0) {
    return 1;
  }
  iVar3 = FUN_0053e290(iVar3 + -1);
  iVar4 = 0;
  pvVar1 = (void *)FUN_00505ef0();
  this = (void *)0x0;
  while (pvVar2 = pvVar1, pvVar2 != (void *)0x0) {
    if (this != (void *)0x0) goto LAB_00556c4c;
    if ((*(byte *)((int)pvVar2 + 0x50) & 0x40) == 0) {
LAB_00556c3f:
      pvVar1 = (void *)FUN_0051a5d0(pvVar2);
    }
    else {
      if (iVar3 != iVar4) {
        iVar4 = iVar4 + 1;
        goto LAB_00556c3f;
      }
      pvVar1 = (void *)FUN_0051a5d0(pvVar2);
      this = pvVar2;
    }
  }
  if (this == (void *)0x0) {
    return 1;
  }
LAB_00556c4c:
  iVar3 = FUN_0050cc70(this,param_1);
  return iVar3;
}


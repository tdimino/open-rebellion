
int FUN_0055f730(void *param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar3;
  
  iVar3 = 1;
  for (pvVar2 = (void *)FUN_0051cad0(0x30); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    bVar1 = FUN_00562520(pvVar2,param_1);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (iVar3 == 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
  }
  for (pvVar2 = (void *)FUN_0051cad0(0x38); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    bVar1 = FUN_00562520(pvVar2,param_1);
    if ((CONCAT31(extraout_var_00,bVar1) == 0) || (iVar3 == 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
  }
  return iVar3;
}


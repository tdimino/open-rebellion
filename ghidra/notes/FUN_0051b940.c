
undefined4 __thiscall FUN_0051b940(void *this,int param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = param_1;
  pvVar1 = (void *)FUN_00539fd0(this,1);
  iVar2 = FUN_0051b990(pvVar1,iVar2);
  pvVar1 = (void *)FUN_00539fd0(this,2);
  iVar3 = FUN_0051b990(pvVar1,param_1);
  if ((iVar3 != 0) && (iVar2 != 0)) {
    return 1;
  }
  return 0;
}


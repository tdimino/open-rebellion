
undefined4 __thiscall FUN_0058a530(void *this,uint *param_1,uint *param_2)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_0058ac20(param_1,(void *)((int)this + 0x50));
  iVar3 = FUN_0058ac20(param_1,(void *)((int)this + 0x58));
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0058ac20(param_2,(void *)((int)this + 0x68));
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0058ac20(param_2,(void *)((int)this + 0x60));
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}


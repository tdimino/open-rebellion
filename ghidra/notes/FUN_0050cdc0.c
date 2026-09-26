
undefined4 __thiscall FUN_0050cdc0(void *this,void *param_1)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  
  uVar1 = 1;
  if ((*(int *)((int)this + 0x5c) != 0) || (*(int *)((int)this + 100) != 0)) {
    iVar2 = FUN_0050ab30(this,1,param_1);
    iVar3 = FUN_0050ab30(this,0,param_1);
    if ((iVar3 != 0) && (iVar2 != 0)) {
      return 1;
    }
    uVar1 = 0;
  }
  return uVar1;
}


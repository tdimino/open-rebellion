
undefined4 __thiscall FUN_0058d770(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  
  if ((*(int *)((int)this + 0x10) != 0) && (*(int *)((int)this + 0x14) != 0)) {
    iVar2 = FUN_0058d990(this,param_1);
    uVar3 = FUN_0058dc10(this,param_1);
    if ((uVar3 == 0) || (iVar2 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0058df60(this,param_1);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0058e320(this,param_1);
    if ((iVar2 != 0) && (bVar1)) {
      return 1;
    }
  }
  return 0;
}


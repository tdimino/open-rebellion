
undefined4 __thiscall FUN_00511780(void *this,int param_1,void *param_2)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  bVar1 = true;
  if (param_1 != 0) {
    iVar2 = FUN_0050d000(this,param_1);
    iVar3 = FUN_0050ceb0(this,0);
    if ((iVar3 == 0) || (iVar2 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0050cfd0(this,0);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0050a040(this,0,param_2);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0050a0b0(this,0,param_2);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
  }
  iVar2 = FUN_0053f9c0(0x304,this,param_1,param_2);
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}


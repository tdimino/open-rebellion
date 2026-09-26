
undefined4 __thiscall FUN_00510f20(void *this,uint param_1,void *param_2)

{
  bool bVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  
  uVar2 = 1;
  if (DAT_006b90e0 != 0) {
    iVar3 = FUN_0050bbb0(this,param_2);
    iVar4 = FUN_0050c180(this,param_2);
    if ((iVar4 == 0) || (iVar3 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0050c1b0(this,param_2);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0050c1e0(this,param_2);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0050b5a0(this,param_2);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0050ba90(this,param_2);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0050b4d0(this,param_2);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0050bb00(this,param_2);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0053f9c0(0x303,this,param_1,param_2);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    uVar5 = FUN_0053fa60(900,param_1,this,(int *)(*(int *)((int)this + 0x54) + 0x20),param_2);
    if ((uVar5 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    uVar5 = FUN_0053fa60(0x38d,param_1,this,(int *)(*(int *)((int)this + 0x54) + 0x38),param_2);
    if ((uVar5 != 0) && (bVar1)) {
      return 1;
    }
    uVar2 = 0;
  }
  return uVar2;
}


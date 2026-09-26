
int __thiscall FUN_005109f0(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  void *pvVar1;
  bool bVar2;
  undefined3 extraout_var;
  uint uVar3;
  int iVar4;
  
  pvVar1 = param_3;
  iVar4 = 1;
  if (DAT_006b90e0 != 0) {
    bVar2 = FUN_0050b610(this,param_3);
    iVar4 = FUN_0050b5a0(this,pvVar1);
    if ((iVar4 == 0) || (CONCAT31(extraout_var,bVar2) == 0)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar4 = FUN_0050b230(this,pvVar1);
    if ((iVar4 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar4 = FUN_0050b2c0(this,pvVar1);
    if ((iVar4 == 0) || (!bVar2)) {
      iVar4 = 0;
    }
    else {
      iVar4 = 1;
    }
    if ((*(byte *)((int)this + 0x88) & 0x20) != 0) {
      uVar3 = FUN_00509980(this,(uint *)&param_3);
      if ((uVar3 == 0) || (iVar4 == 0)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      uVar3 = DAT_006bb3d8;
      if (param_3 == (void *)0x0) {
        uVar3 = DAT_006bb3fc;
      }
      uVar3 = FUN_0053fab0(0x386,uVar3,this,(int *)(*(int *)((int)this + 0x54) + 0x30),pvVar1);
      if ((uVar3 != 0) && (bVar2)) {
        return 1;
      }
      iVar4 = 0;
    }
  }
  return iVar4;
}



undefined4 __thiscall FUN_00511300(void *this,uint param_1,void *param_2)

{
  bool bVar1;
  void *pvVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  
  pvVar2 = param_2;
  uVar3 = 1;
  if (DAT_006b90e0 != 0) {
    iVar4 = FUN_0050c0b0(this,param_2);
    iVar5 = FUN_0050bbb0(this,pvVar2);
    if ((iVar5 == 0) || (iVar4 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar4 = FUN_0050c180(this,pvVar2);
    if ((iVar4 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar4 = FUN_0050c1b0(this,pvVar2);
    if ((iVar4 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar4 = FUN_0050c1e0(this,pvVar2);
    if ((iVar4 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar4 = FUN_0050b310(this,pvVar2);
    if ((iVar4 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if ((bVar1) && (param_1 != 0)) {
      iVar4 = FUN_0050ce20(this,0,1);
      iVar5 = FUN_0050ce20(this,0,2);
      if ((iVar5 == 0) || (iVar4 == 0)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      iVar4 = FUN_0050ce20(this,0,3);
      if ((iVar4 == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      iVar4 = FUN_0050ce50(this,0,1);
      if ((iVar4 == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      iVar4 = FUN_0050ce50(this,0,2);
      if ((iVar4 == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      iVar4 = FUN_0050ce50(this,0,3);
      if ((iVar4 == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      if ((bVar1) && ((*(byte *)((int)this + 0x88) & 0x20) != 0)) {
        uVar6 = FUN_00509890(this,(uint *)&param_2);
        iVar4 = FUN_0050ce80(this,(int)param_2);
        if ((iVar4 == 0) || (uVar6 == 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
      }
    }
    iVar4 = FUN_0053f9c0(0x305,(int *)this,param_1,pvVar2);
    if ((iVar4 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    uVar6 = FUN_0053fa60(0x386,param_1,this,(int *)(*(int *)((int)this + 0x54) + 0x30),pvVar2);
    if ((uVar6 != 0) && (bVar1)) {
      return 1;
    }
    uVar3 = 0;
  }
  return uVar3;
}



bool __thiscall FUN_004fe630(void *this,void *param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined3 extraout_var;
  int iVar3;
  int iVar4;
  bool bVar5;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006404e8;
  pvStack_c = ExceptionList;
  bVar5 = true;
  ExceptionList = &pvStack_c;
  pvVar2 = FUN_00502e30(local_2c,this,3,*(uint *)((int)this + 0x24) >> 6 & 3);
  local_4 = 0;
  bVar1 = FUN_005131b0((int)pvVar2);
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  if (CONCAT31(extraout_var,bVar1) == 0) {
    if ((*(byte *)((int)this + 0x58) & 1) != 0) {
      iVar3 = (**(code **)(*(int *)this + 0xac))(0x14,param_1);
      bVar5 = iVar3 != 0;
    }
  }
  else {
    iVar3 = FUN_004fe230(this,1,param_1);
    iVar4 = FUN_004f7480(this,1,param_1);
    if ((iVar4 == 0) || (iVar3 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_004f74f0(this,1,param_1);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
  }
  ExceptionList = pvStack_c;
  return bVar5;
}


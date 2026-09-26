
uint __thiscall FUN_004f4850(void *this,void *param_1,int *param_2)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  undefined4 local_24;
  int local_20 [4];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063f7a0;
  pvStack_c = ExceptionList;
  local_24 = 1;
  local_20[0] = -1;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_24);
  local_4 = 0xffffffff;
  FUN_00619730();
  (**(code **)(*param_2 + 4))();
  uVar1 = (**(code **)(*(int *)this + 0x1c))(param_1);
  iVar2 = 0;
  if ((uVar1 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
    iVar2 = FUN_0041c210(*(int *)((int)this + 0x40));
    uVar1 = (uint)(iVar2 != 0);
  }
  iVar3 = 0;
  if ((uVar1 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
    iVar3 = iVar2;
  }
  iVar2 = thunk_FUN_00506e40();
  if ((iVar2 == 0) || (uVar1 == 0)) {
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  FUN_00541e70(local_20);
  puStack_8 = (undefined1 *)0x1;
  FUN_0051fcb0(this,local_20);
  if ((((uVar1 != 0) && (iVar2 != 0)) && (iVar3 != 0)) && (*(int *)((int)param_1 + 4) == -1)) {
    uVar1 = (**(code **)(*(int *)this + 0x28))();
    uVar1 = FUN_005425e0(local_20,(uint *)(iVar3 + 0x60),uVar1,(int)this + 0x58,param_2,param_1);
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_10;
  return uVar1;
}


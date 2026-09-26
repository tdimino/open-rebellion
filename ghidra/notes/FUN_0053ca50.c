
bool __thiscall FUN_0053ca50(void *this,int param_1,int *param_2,void *param_3)

{
  short sVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  bool bVar4;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00646527;
  pvStack_c = ExceptionList;
  bVar4 = true;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_2 = 0;
  sVar1 = FUN_005f50e0(param_1);
  if (sVar1 == 0) {
LAB_0053cb53:
    if (bVar4) goto LAB_0053cb87;
  }
  else {
    puVar2 = (undefined4 *)FUN_00618b70(0x44);
    if (puVar2 == (undefined4 *)0x0) {
      FUN_00619730();
    }
    local_4 = 1;
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      puVar2 = FUN_0053d020(puVar2);
    }
    local_4 = 0xffffffff;
    bVar4 = puVar2 != (undefined4 *)0x0;
    *param_2 = (int)puVar2;
    if (bVar4) {
      FUN_005f5600(puVar2,*(undefined4 *)(*(int *)((int)this + 0x44) + 8));
      *(undefined4 *)(*param_2 + 0x1c) = *(undefined4 *)((int)this + 0x1c);
      *(undefined4 *)(*param_2 + 0x20) = *(undefined4 *)((int)this + 0x20);
      (**(code **)(*(int *)*param_2 + 0x24))(param_1);
      uVar3 = FUN_0053ca30(this,&stack0x00000000);
      puStack_8 = (undefined1 *)0x2;
      (**(code **)(*(int *)*param_2 + 0x2c))(uVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      goto LAB_0053cb53;
    }
  }
  local_14 = 1;
  local_10 = 1;
  local_4 = 3;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
LAB_0053cb87:
  if ((*(int *)((int)param_3 + 4) != -1) && ((undefined4 *)*param_2 != (undefined4 *)0x0)) {
    (*(code *)**(undefined4 **)*param_2)(1);
    *param_2 = 0;
  }
  ExceptionList = pvStack_c;
  return bVar4;
}


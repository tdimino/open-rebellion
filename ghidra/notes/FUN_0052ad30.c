
uint __thiscall FUN_0052ad30(void *this,void *param_1)

{
  uint *puVar1;
  void *pvVar2;
  int iVar3;
  undefined *puVar4;
  uint uVar5;
  undefined1 local_28 [4];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644a48;
  local_c = ExceptionList;
  uVar5 = 1;
  ExceptionList = &local_c;
  FUN_005f2f50(local_24);
  local_4 = 0;
  puVar1 = (uint *)FUN_0040d760(this,local_28);
  local_4._0_1_ = 1;
  puVar1 = FUN_004ece40(puVar1);
  local_4._0_1_ = 0;
  FUN_00619730();
  if (puVar1 == (uint *)0x0) {
    pvVar2 = FUN_005f35b0(local_18,&DAT_006b120c);
    local_4._0_1_ = 2;
    FUN_005f3090(local_24,(int)pvVar2);
    local_4._0_1_ = 0;
    FUN_005f2ff0(local_18);
  }
  else {
    iVar3 = thunk_FUN_0052be30(this);
    uVar5 = (uint)(iVar3 != 0);
    if (uVar5 == 0) goto LAB_0052adfd;
    puVar4 = FUN_004f62d0(iVar3);
    FUN_005f3090(local_24,(int)puVar4);
  }
  if (uVar5 != 0) {
    uVar5 = FUN_00529980(this,(int)local_24,param_1);
  }
LAB_0052adfd:
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_24);
  ExceptionList = local_c;
  return uVar5;
}


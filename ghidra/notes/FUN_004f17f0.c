
uint __thiscall FUN_004f17f0(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  undefined4 uVar1;
  void *pvVar2;
  void *pvVar3;
  uint uVar4;
  int *piVar5;
  uint uVar6;
  bool bVar7;
  undefined4 local_1c;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar2 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f000;
  local_c = ExceptionList;
  bVar7 = true;
  uVar1 = *(undefined4 *)(*(int *)((int)this + 0x2c) + 0x30);
  local_1c._0_2_ = (short)uVar1;
  local_1c._2_2_ = (undefined2)((uint)uVar1 >> 0x10);
  switch(*(undefined2 *)((int)this + 0x96)) {
  case 0:
    break;
  case 1:
    local_1c = CONCAT22(local_1c._2_2_,(short)local_1c + 0x6800);
    uVar1 = local_1c;
    break;
  case 2:
    local_1c = CONCAT22(local_1c._2_2_,(short)local_1c + 0x6c00);
    uVar1 = local_1c;
    break;
  case 3:
    local_1c = CONCAT22(local_1c._2_2_,(short)local_1c + 0x7000);
    uVar1 = local_1c;
    break;
  default:
    bVar7 = false;
  }
  local_1c = uVar1;
  uVar4 = 0;
  ExceptionList = &local_c;
  if (bVar7) {
    ExceptionList = &local_c;
    pvVar3 = FUN_005f2fc0(local_18,&local_1c);
    local_4 = 0;
    uVar4 = FUN_004f6e60(this,(int)pvVar3,pvVar2);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
  }
  uVar6 = 0;
  if (uVar4 != 0) {
    piVar5 = (int *)FUN_004ece30(&param_3);
    local_4 = 1;
    uVar6 = FUN_004ee740(this,piVar5,pvVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return uVar6;
}


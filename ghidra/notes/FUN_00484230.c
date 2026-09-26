
void * __thiscall
FUN_00484230(void *this,void *param_1,uint param_2,undefined4 param_3,void *param_4,
            undefined4 param_5)

{
  uint *this_00;
  void *pvVar1;
  uint *puVar2;
  undefined4 *puVar3;
  int iVar4;
  uint uVar5;
  uint local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634641;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  puVar2 = FUN_004025b0(param_1,&local_14);
  local_4 = 0;
  FUN_004ecc70(this,puVar2);
  local_4._0_1_ = 2;
  FUN_00619730();
  puVar2 = (uint *)((int)this + 0x24);
  FUN_005f4950(puVar2,0);
  this_00 = (uint *)((int)this + 0x28);
  local_4._0_1_ = 3;
  FUN_0042dbe0(this_00);
  local_4._0_1_ = 4;
  FUN_004ece30((undefined4 *)((int)this + 0x2c));
  local_4._0_1_ = 5;
  *(undefined ***)this = &PTR_FUN_0065aa50;
  FUN_005f4950(&param_1,0);
  *(uint *)((int)this + 0x20) = param_2;
  local_4._0_1_ = 6;
  puVar3 = (undefined4 *)FUN_00402e40(*(void **)((int)pvVar1 + 0x2c),&param_2);
  local_4._0_1_ = 7;
  FUN_004f26d0(this_00,puVar3);
  local_4._0_1_ = 6;
  FUN_00619730();
  *(undefined4 *)((int)this + 0x38) = param_3;
  *(void **)((int)this + 0x3c) = param_4;
  *(undefined4 *)((int)this + 0x40) = param_5;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  iVar4 = FUN_0042e190(param_4,this_00);
  param_1 = *(void **)(iVar4 + 0x18);
  uVar5 = *puVar2 | (uint)param_1 & 0x4ff;
  *puVar2 = uVar5;
  *puVar2 = uVar5 | 0x20000;
  puVar3 = (undefined4 *)((int)this + 0x44);
  for (iVar4 = 0x1e; iVar4 != 0; iVar4 = iVar4 + -1) {
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
  }
  *(undefined4 *)((int)this + 0x1c) = 1;
  local_4 = CONCAT31(local_4._1_3_,5);
  FUN_00619730();
  ExceptionList = local_c;
  return this;
}


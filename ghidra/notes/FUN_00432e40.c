
void * __thiscall
FUN_00432e40(void *this,undefined4 *param_1,void *param_2,undefined4 param_3,undefined4 param_4)

{
  uint uVar1;
  void *pvVar2;
  uint *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined4 local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d0ec;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  puVar3 = FUN_004025b0(param_2,(uint *)&param_2);
  local_4 = 0;
  FUN_004ecc70(this,puVar3);
  local_4._0_1_ = 2;
  FUN_00619730();
  FUN_004ece30((undefined4 *)((int)this + 0x28));
  local_4._0_1_ = 3;
  FUN_004ece30((undefined4 *)((int)this + 0x2c));
  puVar3 = (uint *)((int)this + 0x38);
  local_4._0_1_ = 4;
  FUN_005f4950(puVar3,0);
  local_4._0_1_ = 5;
  FUN_004f5510((undefined4 *)((int)this + 0x48));
  local_4 = CONCAT31(local_4._1_3_,6);
  *(undefined ***)this = &PTR_FUN_00658d50;
  FUN_004ece30(&local_14);
  *(undefined4 *)((int)this + 0x20) = *param_1;
  *(undefined4 *)((int)this + 0x44) = 0;
  uVar1 = *(uint *)((int)pvVar2 + 0x24);
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(uint *)((int)this + 0x24) = uVar1 >> 6 & 3;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x40) = param_3;
  *(undefined4 *)((int)this + 200) = param_4;
  *(undefined4 *)((int)this + 0x1c) = 1;
  *puVar3 = *puVar3 | 4;
  puVar5 = (undefined4 *)((int)this + 0x50);
  for (iVar4 = 0x1e; iVar4 != 0; iVar4 = iVar4 + -1) {
    *puVar5 = 0;
    puVar5 = puVar5 + 1;
  }
  FUN_00619730();
  ExceptionList = local_c;
  return this;
}


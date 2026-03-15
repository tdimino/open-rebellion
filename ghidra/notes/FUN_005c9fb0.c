
void * __thiscall FUN_005c9fb0(void *this,uint param_1,undefined4 param_2)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006540ad;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005a6210((undefined4 *)this);
  puVar1 = (undefined4 *)((int)this + 0x1c);
  local_4 = 0;
  FUN_005f52c0(puVar1);
  *puVar1 = &PTR_FUN_0066bdd0;
  *(undefined ***)((int)this + 0x28) = &PTR_FUN_0066bacc;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined ***)((int)this + 0x30) = &PTR_FUN_0066bac8;
  *(undefined4 *)((int)this + 0x34) = 0;
  local_4._0_1_ = 3;
  FUN_005f52c0((undefined4 *)((int)this + 0x44));
  *(undefined4 *)((int)this + 0x44) = &PTR_FUN_0066bdd0;
  local_4._0_1_ = 4;
  FUN_005c6150((undefined4 *)((int)this + 0x74));
  *(undefined4 *)((int)this + 0x74) = &PTR_FUN_0066bdb8;
  local_4._0_1_ = 5;
  FUN_005c6150((undefined4 *)((int)this + 0x80));
  *(undefined4 *)((int)this + 0x80) = &PTR_FUN_0066bdb8;
  local_4._0_1_ = 6;
  FUN_005c6150((undefined4 *)((int)this + 0x8c));
  *(undefined4 *)((int)this + 0x8c) = &PTR_FUN_0066cda0;
  local_4._0_1_ = 7;
  FUN_005c6150((undefined4 *)((int)this + 0x98));
  *(undefined4 *)((int)this + 0x98) = &PTR_FUN_0066cda0;
  local_4._0_1_ = 8;
  FUN_005c6150((undefined4 *)((int)this + 0xa4));
  *(undefined4 *)((int)this + 0xa4) = &PTR_FUN_0066cda0;
  local_4._0_1_ = 9;
  *(undefined ***)this = &PTR_FUN_0066cd28;
  *puVar1 = &PTR_LAB_0066cd10;
  *(undefined4 *)((int)this + 0xb0) = 0;
  *(undefined4 *)((int)this + 0xb4) = 0;
  *(undefined4 *)((int)this + 0x38) = 2;
  *(undefined4 *)((int)this + 0x3c) = 2;
  puVar1 = (undefined4 *)FUN_00618b70(0x3c);
  local_4._0_1_ = 10;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005cbd20(puVar1,this);
    *puVar1 = &PTR_FUN_0066ccd0;
    puVar1[1] = 2;
  }
  *(undefined4 **)((int)this + 0x40) = puVar1;
  local_4._0_1_ = 9;
  *(undefined4 *)((int)this + 0x60) = 0;
  *(uint *)((int)this + 0x70) = param_1;
  *(undefined4 *)((int)this + 0x6c) = 0;
  *(undefined4 *)((int)this + 100) = 0;
  *(undefined4 *)((int)this + 0x58) = 0;
  *(undefined4 *)((int)this + 0x5c) = 0;
  *(undefined4 *)((int)this + 0x54) = 0;
  FUN_005f5600(this,param_2);
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)FUN_00618b70(0x48);
    local_4._0_1_ = 0xb;
    if (puVar1 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar1);
    }
    local_4._0_1_ = 9;
  }
  iVar2 = FUN_005a6340(DAT_006bc470,param_1);
  if (iVar2 != 0) {
    FUN_005c5f20((void *)((int)this + 0x44),iVar2);
    *(undefined4 *)(iVar2 + 0x178) = *(undefined4 *)((int)this + 0x18);
    *(undefined4 *)((int)this + 0x50) = *(undefined4 *)(iVar2 + 0x168);
    *(undefined4 *)((int)this + 0x2c) = *(undefined4 *)(iVar2 + 0x11c);
    *(undefined4 *)((int)this + 0x34) = *(undefined4 *)(iVar2 + 0x124);
  }
  ExceptionList = local_c;
  return this;
}


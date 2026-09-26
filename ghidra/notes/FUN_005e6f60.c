
void __fastcall FUN_005e6f60(void *param_1)

{
  void *pvVar1;
  undefined4 uVar2;
  uint *this;
  uint *puVar3;
  uint uVar4;
  undefined1 uVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006550e6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    this = (uint *)0x0;
  }
  else {
    uVar5 = 2;
    uVar4 = 0x516;
    uVar2 = FUN_006037f0(6);
    this = FUN_005fbd20(pvVar1,uVar2,uVar4,uVar5);
  }
  local_4 = 0xffffffff;
  pvVar1 = (void *)FUN_00618b70(0x28);
  local_4 = 1;
  if (pvVar1 == (void *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = FUN_005fbda0(pvVar1,0x60,0x27,this);
  }
  local_4 = 0xffffffff;
  *(uint **)((int)param_1 + 0x9c) = puVar3;
  if (puVar3 != (uint *)0x0) {
    FUN_005fcc30(this,puVar3,0,0,0x1a,0x11,(undefined4 *)0x60,0x27);
  }
  FUN_005ff2d0(param_1,this,0,0);
  ExceptionList = local_c;
  return;
}


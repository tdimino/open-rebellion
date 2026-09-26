
undefined4 __fastcall FUN_004c6f00(int param_1)

{
  void *this;
  uint *this_00;
  bool bVar1;
  undefined4 *puVar2;
  void *pvVar3;
  undefined4 uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ac30;
  local_c = ExceptionList;
  local_10 = 0;
  if (*(int *)(*(int *)(param_1 + 0x28) + 0xa8) < *(int *)(*(int *)(param_1 + 0x28) + 0xac)) {
    uVar5 = 0x180;
    uVar7 = 0x60;
    uVar6 = *(uint *)(param_1 + 0x44) & 0xffffff | 0x2c000000;
    ExceptionList = &local_c;
    *(uint *)(param_1 + 0x44) = uVar6;
    *(uint *)(param_1 + 0x44) = CONCAT31((uint3)(uVar6 >> 8) & 0xff0000,1);
  }
  else {
    uVar7 = 0x20;
    ExceptionList = &local_c;
    *(uint *)(param_1 + 0x44) = *(uint *)(param_1 + 0x44) & 0xffffff | 0x2d000000;
    uVar5 = 0x80;
    *(undefined4 *)(param_1 + 0x44) = 0x2d000002;
  }
  puVar2 = FUN_0049d770(*(void **)(param_1 + 0x28),(undefined4 *)(param_1 + 0x40),uVar5,0,0,0,2);
  this = (void *)(param_1 + 0x48);
  FUN_00435790(this,(int)puVar2);
  puVar2 = FUN_004357b0(this,&local_18);
  local_4 = 0;
  this_00 = (uint *)(param_1 + 0x3c);
  FUN_004f26d0(this_00,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(this);
  uVar7 = uVar7 | 4;
  puVar2 = FUN_0049da70(*(void **)(param_1 + 0x28),this_00,(undefined4 *)(param_1 + 0x40),uVar7,0,2,
                        0,(undefined4 *)&DAT_00000002);
  FUN_00435790(this,(int)puVar2);
  pvVar3 = FUN_0049da10(*(void **)(param_1 + 0x28),this_00,uVar7,0,2,0,5,&DAT_00000002);
  FUN_00435790(this,(int)pvVar3);
  pvVar3 = FUN_0049da10(*(void **)(param_1 + 0x28),this_00,uVar7,0,2,0,6,&DAT_00000002);
  FUN_00435790(this,(int)pvVar3);
  pvVar3 = FUN_0049da10(*(void **)(param_1 + 0x28),this_00,uVar7,0,2,0,7,&DAT_00000002);
  FUN_00435790(this,(int)pvVar3);
  puVar2 = FUN_004357b0(this,&local_18);
  local_4 = 1;
  FUN_004f26d0(this_00,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(this);
  local_18 = 0x90;
  local_14 = 0x98;
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  uVar4 = 1;
  if (!bVar1) {
    uVar4 = local_10;
  }
  ExceptionList = local_c;
  return uVar4;
}


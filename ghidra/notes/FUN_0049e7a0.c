
/* WARNING: Type propagation algorithm not settling */

int __fastcall FUN_0049e7a0(void *param_1)

{
  uint *this;
  bool bVar1;
  undefined4 *puVar2;
  int *piVar3;
  void *pvVar4;
  uint *puVar5;
  int iVar6;
  uint local_24 [3];
  uint local_18;
  int local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006372b0;
  local_c = ExceptionList;
  local_14[0] = 0;
  ExceptionList = &local_c;
  FUN_0049eb80(param_1,local_24);
  puVar2 = FUN_0049d770(*(void **)((int)param_1 + 0x28),(undefined4 *)((int)param_1 + 0x3c),
                        local_24[0] | 0x10,0,0,0,2);
  pvVar4 = (void *)((int)param_1 + 0x44);
  FUN_00435790(pvVar4,(int)puVar2);
  puVar2 = FUN_004357b0(pvVar4,local_24 + 1);
  local_4 = 0;
  this = (uint *)((int)param_1 + 0x40);
  FUN_004f26d0(this,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  FUN_0049ebc0(param_1,local_24 + 3);
  puVar2 = FUN_0049da70(*(void **)((int)param_1 + 0x28),this,(undefined4 *)((int)param_1 + 0x3c),
                        local_18 | 4,0,0,0,(undefined4 *)&DAT_00000002);
  FUN_00435790(pvVar4,(int)puVar2);
  puVar2 = FUN_004357b0(pvVar4,local_24 + 1);
  local_4 = 1;
  FUN_004f26d0(this,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  local_24[1] = 0x90;
  local_24[2] = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  iVar6 = local_14[0];
  if (((bVar1) &&
      (piVar3 = FUN_004f3220(*(int *)((int)param_1 + 0x2c),this), iVar6 = local_14[0],
      piVar3 != (int *)0x0)) &&
     (pvVar4 = (void *)FUN_0049ec00(param_1,piVar3), iVar6 = local_14[0], pvVar4 != (void *)0x0)) {
    puVar5 = FUN_004025b0(pvVar4,local_24 + 1);
    local_4 = 2;
    FUN_004f26d0(this,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    iVar6 = 1;
  }
  if (iVar6 == 0) {
    local_14[0] = 0xa0;
    local_14[1] = 3;
    local_4 = 3;
    (**(code **)(**(int **)((int)param_1 + 0x54) + 0xc))(0,local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar6;
}



/* WARNING: Type propagation algorithm not settling */

int __thiscall FUN_00586150(void *this,void *param_1)

{
  bool bVar1;
  uint *puVar2;
  int unaff_EBX;
  uint uVar3;
  uint local_18 [2];
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064f678;
  pvStack_c = ExceptionList;
  local_18[0] = 1;
  local_18[1] = 1;
  local_10 = (void *)0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,local_18 + 1);
  local_4 = 0xffffffff;
  FUN_00619730();
  uVar3 = *(uint *)((int)this + 0x3c) >> 0x18;
  puVar2 = (uint *)(**(code **)(*(int *)this + 0x28))(local_18 + 1);
  if ((uVar3 < *puVar2) || (puVar2[1] <= uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    local_18[0] = 1;
    local_18[1] = 0x17;
    puStack_8 = (undefined1 *)0x1;
    FUN_00520580(param_1,local_18);
    puStack_8 = (undefined1 *)0xffffffff;
    FUN_00619730();
  }
  if (*(int *)((int)param_1 + 4) == -1) {
    local_18[0] = 1;
    local_18[1] = 0x12;
    puStack_8 = (undefined1 *)0x2;
    unaff_EBX = FUN_00504e60((uint *)((int)this + 0x3c),(undefined4 *)&stack0x00000000,local_18,
                             param_1);
    puStack_8 = (undefined1 *)0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_10;
  return unaff_EBX;
}


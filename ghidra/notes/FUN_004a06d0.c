
void __thiscall FUN_004a06d0(void *this,uint *param_1,int param_2,uint *param_3)

{
  uint uVar1;
  uint uVar2;
  bool bVar3;
  uint *puVar4;
  int *piVar5;
  int iVar6;
  void *this_00;
  uint *puVar7;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637588;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_2 == 0) {
    ExceptionList = &local_c;
    FUN_004f2d10(*(int *)(*(int *)((int)this + 0x14c) + 0x9c),param_1);
  }
  piVar5 = FUN_004a25c0(this);
  if (piVar5 == (int *)0x0) {
    ExceptionList = local_c;
    return;
  }
  if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  puVar4 = param_3;
  if (!bVar3) {
    ExceptionList = local_c;
    return;
  }
  if (param_3 == (uint *)0x0) {
    ExceptionList = local_c;
    return;
  }
  if ((*param_3 >> 0x18 < 0x40) || (0x7f < *param_3 >> 0x18)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if (!bVar3) {
    ExceptionList = local_c;
    return;
  }
  iVar6 = FUN_0060a860(*(void **)((int)this + 0x1b4),*puVar4 & 0xffffff);
  if (iVar6 == 0) {
    ExceptionList = local_c;
    return;
  }
  if ((*(byte *)(iVar6 + 0x3c) & 1) != 0) {
    FUN_004a0ca0(this);
  }
  FUN_00536da0(local_2c,piVar5,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  if (local_10 != 0) {
    do {
      piVar5 = &param_2;
      this_00 = (void *)FUN_0052bed0((int)local_2c);
      puVar7 = FUN_0042d170(this_00,piVar5);
      uVar1 = *puVar7;
      uVar2 = *puVar4;
      FUN_00619730();
      if (uVar1 == uVar2) break;
      FUN_005130d0((int)local_2c);
    } while (local_10 != 0);
    if (local_10 != 0) goto LAB_004a0841;
  }
  FUN_004a1590(this);
LAB_004a0841:
  local_4 = 0xffffffff;
  FUN_00536ea0(local_2c);
  ExceptionList = local_c;
  return;
}


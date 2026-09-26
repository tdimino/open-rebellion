
void __thiscall FUN_0049e380(void *this,uint *param_1)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  uint uVar4;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00637278;
  local_c = ExceptionList;
  local_14 = 0x90;
  uVar4 = *param_1 >> 0x18;
  local_10 = 0x98;
  bVar1 = false;
  local_4 = 0;
  if ((uVar4 < 0x90) || (ExceptionList = &local_c, 0x97 < uVar4)) {
    local_24 = 8;
    local_20 = 0x10;
    if ((7 < uVar4) && (uVar4 < 0x10)) {
      bVar1 = true;
    }
    ExceptionList = &local_c;
    FUN_00619730();
    if (!bVar1) {
      local_1c = 0x14;
      local_18 = 0x1c;
      if ((uVar4 < 0x14) || (0x1b < uVar4)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        bVar1 = false;
        goto LAB_0049e43a;
      }
    }
  }
  bVar1 = true;
LAB_0049e43a:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    if (*(int *)((int)this + 0x34) == 3) {
      local_14 = 0x90;
      local_10 = 0x98;
      if ((uVar4 < 0x90) || (0x97 < uVar4)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if ((((bVar1) &&
           (piVar2 = FUN_004f3220(*(int *)((int)this + 0x2c),param_1), piVar2 != (int *)0x0)) &&
          (((uint)piVar2[9] >> 6 & 3) != *(uint *)((int)this + 0x2c))) &&
         (iVar3 = FUN_004feea0(piVar2,*(uint *)((int)this + 0x2c),1), iVar3 == 0)) {
        piVar2 = (int *)FUN_004f5cd0(0x214);
        if (piVar2 != (int *)0x0) {
          piVar2[8] = *(int *)((int)this + 0x2c);
          (**(code **)(*piVar2 + 0x2c))(param_1);
          local_24 = 0x24;
          puStack_8 = (undefined1 *)0x1;
          (**(code **)(**(int **)((int)this + 0x54) + 0xc))(piVar2,&stack0xffffffd8);
          local_10 = 0xffffffff;
          FUN_00619730();
          (**(code **)*piVar2)(1);
        }
        *(undefined4 *)((int)this + 0x34) = 0;
        FUN_004ece80((undefined4 *)((int)this + 0x3c));
        FUN_004ece80((undefined4 *)((int)this + 0x40));
        *(undefined4 *)((int)this + 0x38) = 0;
        *(undefined4 *)((int)this + 0x20) = 1;
        *(undefined4 *)((int)this + 0x1c) = 2;
        ExceptionList = local_c;
        return;
      }
    }
    FUN_004f26d0((void *)((int)this + 0x3c),param_1);
    *(undefined4 *)((int)this + 0x1c) = 0;
  }
  else if (uVar4 == 0) {
    *(undefined4 *)((int)this + 0x34) = 0;
    FUN_004ece80((undefined4 *)((int)this + 0x3c));
    FUN_004ece80((undefined4 *)((int)this + 0x40));
    *(undefined4 *)((int)this + 0x38) = 0;
    *(undefined4 *)((int)this + 0x20) = 1;
    *(undefined4 *)((int)this + 0x1c) = 2;
  }
  else {
    local_24 = 1;
    local_20 = 0x27;
    local_4 = 2;
    (**(code **)(**(int **)((int)this + 0x54) + 0xc))(0,&local_24);
    local_4 = 0xffffffff;
    FUN_00619730();
    *(undefined4 *)((int)this + 0x34) = 0;
    FUN_004ece80((undefined4 *)((int)this + 0x3c));
    FUN_004ece80((undefined4 *)((int)this + 0x40));
    *(undefined4 *)((int)this + 0x38) = 0;
    *(undefined4 *)((int)this + 0x20) = 1;
    *(undefined4 *)((int)this + 0x1c) = 2;
  }
  ExceptionList = local_c;
  return;
}


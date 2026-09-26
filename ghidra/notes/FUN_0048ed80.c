
void __thiscall FUN_0048ed80(void *this,int param_1,int param_2)

{
  bool bVar1;
  ushort uVar2;
  int *piVar3;
  int iVar4;
  uint uVar5;
  int *piVar6;
  uint local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006358b0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  local_4 = 0;
  FUN_004ece30(&local_20);
  local_4._0_1_ = 1;
  if (param_1 != 0x1e1) goto switchD_0048ef54_caseD_244;
  FUN_004f26d0(&local_24,(undefined4 *)(param_2 + 0x20));
  FUN_004f26d0((void *)((int)this + 0x5c),&local_24);
  piVar3 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_24);
  *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x412;
  *(ushort *)((int)this + 0x30) = (*(int *)((int)this + 0x20) != 1) + 0x47c;
  if (piVar3 != (int *)0x0) {
    local_1c = 0x30;
    local_18 = 0x3c;
    if ((((local_24 >> 0x18 < 0x30) || (0x3b < local_24 >> 0x18)) || (*(int *)(param_2 + 0x24) == 0)
        ) || (*(int *)(param_2 + 0x28) == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      *(undefined4 *)((int)this + 0x40) = 1;
      if (*(int *)((int)this + 0x20) == 1) {
        iVar4 = FUN_004edc80((int)piVar3);
        if (iVar4 == 2) goto LAB_0048ee94;
      }
      else if ((*(int *)((int)this + 0x20) != 2) || (iVar4 = FUN_004edc80((int)piVar3), iVar4 == 1))
      {
LAB_0048ee94:
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
  }
  if (*(int *)((int)this + 0x40) != 0) {
    uVar5 = FUN_004c5000(piVar3);
    *(short *)((int)this + 0x2c) = (short)uVar5;
    if (piVar3 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = piVar3 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x7180,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (piVar3 == (int *)0x0) {
      piVar6 = (int *)0x0;
    }
    else {
      piVar6 = piVar3 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x50),0x7181,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    local_14 = 0x30;
    local_10 = 0x38;
    if ((local_24 >> 0x18 < 0x30) || (0x37 < local_24 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      switch(local_24 & 0xffffff) {
      case 0x240:
        *(undefined4 *)((int)this + 0x28) = 0x17;
        break;
      case 0x241:
        *(undefined4 *)((int)this + 0x28) = 0x18;
        uVar2 = FUN_004c4990(&DAT_00000134);
        *(ushort *)((int)this + 0x32) = uVar2;
        break;
      case 0x242:
        *(undefined4 *)((int)this + 0x28) = 0x16;
        if ((*(byte *)(piVar3 + 0x2c) & 8) == 0) {
          uVar2 = FUN_004c4990(&DAT_00000132);
          *(ushort *)((int)this + 0x32) = uVar2;
        }
        else {
          uVar2 = FUN_004c4990(&DAT_00000133);
          *(ushort *)((int)this + 0x32) = uVar2;
        }
        break;
      case 0x243:
        *(undefined4 *)((int)this + 0x28) = 0x19;
        break;
      case 0x280:
        *(undefined4 *)((int)this + 0x28) = 0x1a;
        break;
      case 0x281:
        *(undefined4 *)((int)this + 0x28) = 0x1b;
      }
    }
    else {
      *(undefined4 *)((int)this + 0x28) = 0x15;
    }
  }
switchD_0048ef54_caseD_244:
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}



/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall
FUN_005b9c60(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  undefined4 uVar1;
  bool bVar2;
  float fVar3;
  int *piVar4;
  undefined4 *puVar5;
  int iVar6;
  float10 fVar7;
  float10 fVar8;
  float10 fVar9;
  undefined4 in_stack_00000020;
  float in_stack_00000024;
  undefined4 in_stack_00000028;
  undefined4 in_stack_0000002c;
  int in_stack_00000030;
  int in_stack_00000034;
  undefined4 in_stack_00000038;
  float in_stack_0000003c;
  undefined4 in_stack_00000040;
  undefined4 in_stack_00000044;
  undefined4 in_stack_00000048;
  undefined4 in_stack_0000004c;
  int in_stack_00000050;
  undefined4 in_stack_00000054;
  undefined4 in_stack_00000058;
  undefined4 in_stack_0000005c;
  undefined4 in_stack_00000060;
  undefined4 in_stack_00000064;
  undefined4 in_stack_00000068;
  undefined4 in_stack_0000006c;
  undefined4 in_stack_00000070;
  undefined4 in_stack_00000074;
  undefined4 in_stack_00000078;
  undefined4 in_stack_0000007c;
  undefined4 in_stack_00000080;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00653510;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_005b0f70(this);
  *(undefined ***)this = &PTR_FUN_0066c028;
  *(undefined ***)((int)this + 0xd4) = &PTR_LAB_0066c020;
  *(undefined ***)((int)this + 0xf0) = &PTR_LAB_0066c018;
  *(undefined ***)((int)this + 0xfc) = &PTR_LAB_0066c014;
  *(undefined ***)((int)this + 0x104) = &PTR_LAB_0066c008;
  *(undefined ***)((int)this + 0x10c) = &PTR_LAB_0066c000;
  *(undefined4 *)((int)this + 0x128) = param_1;
  *(undefined4 *)((int)this + 300) = param_2;
  *(undefined4 *)((int)this + 0x168) = param_3;
  *(undefined4 *)((int)this + 0x140) = 1;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined4 *)((int)this + 0x13c) = param_4;
  FUN_005f3090((void *)((int)this + 0x144),(int)&stack0x00000014);
  *(undefined4 *)((int)this + 0x164) = in_stack_00000020;
  *(float *)((int)this + 0x3f8) = in_stack_00000024 * _DAT_0066c6c0;
  *(undefined4 *)((int)this + 0x3fc) = in_stack_00000028;
  *(float *)((int)this + 0x3c8) = (float)in_stack_00000030 * _DAT_0066c6b8;
  *(float *)((int)this + 0x3cc) = (float)in_stack_00000034 * _DAT_0066c6b8;
  bVar2 = in_stack_0000003c != _DAT_0066c6d0;
  *(undefined4 *)((int)this + 0x3f0) = in_stack_00000038;
  *(undefined4 *)((int)this + 0x398) = in_stack_00000070;
  *(undefined4 *)((int)this + 0x394) = in_stack_00000074;
  *(undefined4 *)((int)this + 0x3a0) = in_stack_00000078;
  *(undefined4 *)((int)this + 0x134) = in_stack_0000007c;
  *(undefined4 *)((int)this + 0x138) = in_stack_00000080;
  *(undefined4 *)((int)this + 0x3bc) = in_stack_0000002c;
  *(undefined4 *)((int)this + 0x3c0) = in_stack_0000002c;
  *(float *)((int)this + 0x3ec) = in_stack_0000003c;
  *(uint *)((int)this + 0x16c) = (uint)bVar2;
  *(undefined4 *)((int)this + 0x17c) = in_stack_00000040;
  piVar4 = (int *)((int)this + 0x180);
  *(undefined4 *)((int)this + 0x184) = in_stack_00000060;
  *piVar4 = in_stack_00000050;
  *(undefined4 *)((int)this + 0x264) = in_stack_00000044;
  *(undefined4 *)((int)this + 0x26c) = in_stack_00000064;
  *(undefined4 *)((int)this + 0x268) = in_stack_00000054;
  *(undefined4 *)((int)this + 0x2d8) = in_stack_00000048;
  *(undefined4 *)((int)this + 0x2e0) = in_stack_00000068;
  *(undefined4 *)((int)this + 0x2dc) = in_stack_00000058;
  *(undefined4 *)((int)this + 0x1f0) = in_stack_0000004c;
  *(undefined4 *)((int)this + 0x1f8) = in_stack_0000006c;
  *(undefined4 *)((int)this + 500) = in_stack_0000005c;
  *(undefined4 *)((int)this + 0x398) = in_stack_00000070;
  *(undefined4 *)((int)this + 0x394) = in_stack_00000074;
  *(undefined4 *)((int)this + 0x3a0) = in_stack_00000078;
  *(undefined4 *)((int)this + 0x174) = 0;
  *(undefined4 *)((int)this + 0x178) = 0;
  *(undefined4 *)((int)this + 0x130) = 0;
  *(undefined4 *)((int)this + 0x3b4) = 0;
  *(undefined4 *)((int)this + 0x3d4) = 0;
  puVar5 = (undefined4 *)((int)this + 0x34c);
  iVar6 = 4;
  do {
    fVar3 = (float)piVar4[-1] * _DAT_0066c6a8 +
            (float)piVar4[1] * _DAT_0066c6b0 + (float)*piVar4 * _DAT_0066c6ac;
    piVar4[0x1a] = (int)fVar3;
    *(float *)((int)this + 0x3d4) = fVar3 + *(float *)((int)this + 0x3d4);
    *puVar5 = 0xffffffff;
    if ((float)piVar4[0x1a] <= _DAT_0066c6d0) {
      piVar4[0x18] = 0;
    }
    else {
      piVar4[0x18] = 1;
    }
    puVar5 = puVar5 + 1;
    piVar4 = piVar4 + 0x1d;
    iVar6 = iVar6 + -1;
  } while (iVar6 != 0);
  uVar1 = *(undefined4 *)((int)this + 0x3d4);
  *(undefined4 *)((int)this + 0x3e4) = 0x3e800000;
  *(undefined4 *)((int)this + 0x3d8) = uVar1;
  *(undefined4 *)((int)this + 0x3e0) = uVar1;
  *(undefined4 *)((int)this + 0x3dc) = uVar1;
  *(undefined4 *)((int)this + 1000) = 0x3f800000;
  *(undefined4 *)((int)this + 0x3f4) = 0;
  fVar7 = (float10)fsin((float10)_DAT_0066c6a0 * (float10)_DAT_0066c6ec);
  *(float *)((int)this + 4) = (float)fVar7;
  *(float *)((int)this + 8) = DAT_0066c6bc;
  fVar3 = DAT_0066c6bc * _DAT_0066c6f0;
  *(undefined4 *)((int)this + 0x37c) = 0;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(float *)((int)this + 0xc) = fVar3;
  *(float *)((int)this + 0x160) = fVar3 * _DAT_0066c6f4;
  *(undefined4 *)((int)this + 900) = 0;
  *(undefined4 *)((int)this + 0x388) = 0;
  *(undefined4 *)((int)this + 0x38c) = 0;
  *(undefined4 *)((int)this + 0x390) = 0;
  *(undefined4 *)((int)this + 0x3a4) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(undefined4 *)((int)this + 0x150) = 0;
  fVar3 = _DAT_0066c6a0 * _DAT_0066c6f8;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0xa8) = 0xbf800000;
  *(float *)((int)this + 0x9c) = fVar3;
  iVar6 = FUN_00597530();
  if (*(int *)(iVar6 + 0x668) == 0) {
    fVar7 = FUN_005ba270(this);
    if ((float10)_DAT_0066c6e0 < fVar7) {
      fVar8 = FUN_005ba270(this);
      fVar9 = (float10)_DAT_0066c6a0;
      fVar7 = (float10)_DAT_0066c6d8;
      *(undefined4 *)((int)this + 0xa0) = 0;
      *(undefined4 *)((int)this + 0xa4) = 0;
      *(undefined4 *)((int)this + 0xa8) = 0xbf800000;
      *(float *)((int)this + 0x9c) = (float)(fVar9 * fVar7 * fVar8 * (float10)_DAT_0066c6fc);
    }
    else {
      fVar3 = _DAT_0066c6a0 * _DAT_0066c6f8;
      *(undefined4 *)((int)this + 0xa0) = 0;
      *(undefined4 *)((int)this + 0xa4) = 0;
      *(undefined4 *)((int)this + 0xa8) = 0xbf800000;
      *(float *)((int)this + 0x9c) = fVar3;
    }
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x00000014);
  ExceptionList = local_c;
  return this;
}


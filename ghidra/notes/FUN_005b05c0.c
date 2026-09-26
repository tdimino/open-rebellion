
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall
FUN_005b05c0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  undefined4 uVar1;
  bool bVar2;
  float fVar3;
  int iVar4;
  uint uVar5;
  int *piVar6;
  float10 fVar7;
  float10 fVar8;
  undefined4 in_stack_00000020;
  undefined4 in_stack_00000024;
  undefined4 in_stack_00000028;
  undefined4 in_stack_0000002c;
  undefined4 in_stack_00000030;
  undefined4 in_stack_00000034;
  int in_stack_00000038;
  float in_stack_00000040;
  undefined4 in_stack_00000044;
  undefined4 *in_stack_00000048;
  float in_stack_0000004c;
  float in_stack_00000050;
  undefined4 in_stack_00000054;
  undefined4 in_stack_00000058;
  undefined4 in_stack_0000005c;
  undefined4 in_stack_00000060;
  undefined4 in_stack_00000064;
  int in_stack_00000068;
  undefined4 in_stack_0000006c;
  undefined4 in_stack_00000070;
  undefined4 in_stack_00000074;
  undefined4 in_stack_00000078;
  undefined4 in_stack_0000007c;
  undefined4 in_stack_00000080;
  undefined4 in_stack_00000084;
  int in_stack_00000088;
  int in_stack_0000008c;
  undefined4 in_stack_00000090;
  int in_stack_00000094;
  int in_stack_00000098;
  undefined4 in_stack_0000009c;
  undefined4 in_stack_000000a0;
  undefined4 in_stack_000000a4;
  undefined4 in_stack_000000a8;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0065310e;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_005b0f70(this);
  local_4._0_1_ = 1;
  FUN_005b82a0((undefined4 *)((int)this + 0x418));
  *(undefined ***)this = &PTR_FUN_0066c140;
  *(undefined ***)((int)this + 0xd4) = &PTR_LAB_0066c138;
  *(undefined ***)((int)this + 0xf0) = &PTR_LAB_0066c130;
  *(undefined ***)((int)this + 0xfc) = &PTR_LAB_0066c12c;
  *(undefined ***)((int)this + 0x104) = &PTR_LAB_0066c120;
  *(undefined ***)((int)this + 0x10c) = &PTR_LAB_0066c118;
  *(undefined4 *)((int)this + 0x418) = &PTR_LAB_0066c110;
  *(undefined4 *)((int)this + 0x128) = param_1;
  *(undefined4 *)((int)this + 300) = param_2;
  *(undefined4 *)((int)this + 0x168) = param_3;
  *(undefined4 *)((int)this + 0x140) = 0;
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined4 *)((int)this + 0x13c) = param_4;
  FUN_005f3090((void *)((int)this + 0x144),(int)&stack0x00000014);
  *(undefined4 *)((int)this + 0x3c8) = in_stack_00000020;
  *(undefined4 *)((int)this + 0x3cc) = in_stack_00000024;
  *(undefined4 *)((int)this + 0x428) = in_stack_0000002c;
  *(undefined4 *)((int)this + 0x424) = 0;
  *(undefined4 *)((int)this + 0x3b8) = in_stack_00000030;
  *(float *)((int)this + 0x3f8) = in_stack_00000040 * _DAT_0066c474 * _DAT_0066c498;
  bVar2 = in_stack_0000004c != _DAT_0066c484;
  *(undefined4 *)((int)this + 0x3fc) = in_stack_00000044;
  *(undefined4 **)((int)this + 0x3bc) = in_stack_00000048;
  *(undefined4 **)((int)this + 0x3c0) = in_stack_00000048;
  *(float *)((int)this + 0x3ec) = in_stack_0000004c;
  in_stack_00000050 = in_stack_00000050 * _DAT_0066c49c;
  *(uint *)((int)this + 0x16c) = (uint)bVar2;
  *(undefined4 *)((int)this + 0x37c) = in_stack_00000090;
  *(float *)((int)this + 0x3e4) = in_stack_00000050;
  *(undefined4 *)((int)this + 1000) = in_stack_00000054;
  piVar6 = (int *)((int)this + 0x180);
  *(undefined4 *)((int)this + 0x17c) = in_stack_00000058;
  *(undefined4 *)((int)this + 0x184) = in_stack_00000078;
  *piVar6 = in_stack_00000068;
  *(undefined4 *)((int)this + 0x264) = in_stack_0000005c;
  *(undefined4 *)((int)this + 0x26c) = in_stack_0000007c;
  *(undefined4 *)((int)this + 0x268) = in_stack_0000006c;
  *(undefined4 *)((int)this + 0x2d8) = in_stack_00000060;
  *(undefined4 *)((int)this + 0x2e0) = in_stack_00000080;
  *(undefined4 *)((int)this + 0x2dc) = in_stack_00000070;
  *(undefined4 *)((int)this + 0x1f0) = in_stack_00000064;
  *(undefined4 *)((int)this + 0x1f8) = in_stack_00000084;
  *(undefined4 *)((int)this + 500) = in_stack_00000074;
  *(undefined4 *)((int)this + 0x398) = in_stack_0000009c;
  *(undefined4 *)((int)this + 0x394) = in_stack_000000a0;
  *(undefined4 *)((int)this + 0x3a0) = in_stack_000000a4;
  in_stack_00000048 = (undefined4 *)((int)this + 0x34c);
  *(undefined4 *)((int)this + 0x164) = 0;
  *(undefined4 *)((int)this + 0x174) = 0;
  *(undefined4 *)((int)this + 0x178) = 0;
  *(undefined4 *)((int)this + 0x380) = 0;
  *(undefined4 *)((int)this + 0x130) = 0;
  *(undefined4 *)((int)this + 0x3b0) = 0;
  *(undefined4 *)((int)this + 0x3b4) = 0;
  *(undefined4 *)((int)this + 0x3b8) = in_stack_00000030;
  *(undefined4 *)((int)this + 0x39c) = in_stack_00000034;
  *(undefined4 *)((int)this + 0x3ac) = 0;
  *(undefined4 *)((int)this + 0x3d4) = 0;
  uVar5 = 0;
  do {
    fVar3 = (float)*piVar6 * _DAT_0066c464 +
            (float)piVar6[-1] * _DAT_0066c460 + (float)piVar6[1] * _DAT_0066c468;
    piVar6[0x1a] = (int)fVar3;
    *(float *)((int)this + 0x3d4) = fVar3 + *(float *)((int)this + 0x3d4);
    *in_stack_00000048 = 0xffffffff;
    piVar6[0x18] = 0;
    if (_DAT_0066c484 < (float)piVar6[0x1a]) {
      if (*(uint *)((int)this + 0x37c) < 4) {
        piVar6[0x18] = 1;
        piVar6[0x19] = piVar6[0x1a];
      }
      else {
        FUN_005b6980(this,uVar5);
        FUN_005b64e0(this,uVar5);
      }
    }
    uVar5 = uVar5 + 1;
    in_stack_00000048 = in_stack_00000048 + 1;
    piVar6 = piVar6 + 0x1d;
  } while (uVar5 < 4);
  uVar1 = *(undefined4 *)((int)this + 0x3d4);
  *(undefined4 *)((int)this + 0x3f4) = 0;
  *(undefined4 *)((int)this + 0x3d8) = uVar1;
  *(undefined4 *)((int)this + 0x3e0) = uVar1;
  *(undefined4 *)((int)this + 0x3dc) = uVar1;
  *(undefined4 *)((int)this + 0x3f0) = in_stack_00000028;
  *(undefined4 *)((int)this + 0x134) = 0;
  *(undefined4 *)((int)this + 0x138) = 0;
  fVar7 = (float10)_DAT_0066c458;
  fVar8 = (float10)_DAT_0066c4a0;
  *(int *)((int)this + 0x378) = in_stack_00000098;
  fVar8 = (float10)fsin(fVar7 * fVar8);
  *(int *)((int)this + 0x36c) = in_stack_00000088;
  *(int *)((int)this + 0x370) = in_stack_0000008c;
  *(int *)((int)this + 0x374) = in_stack_00000094;
  *(int *)((int)this + 0x158) =
       in_stack_00000098 + in_stack_00000094 + in_stack_0000008c + in_stack_00000088;
  *(float *)((int)this + 4) = (float)fVar8;
  *(undefined4 *)((int)this + 900) = 0;
  *(undefined4 *)((int)this + 0x388) = 0;
  *(undefined4 *)((int)this + 0x38c) = 0;
  *(undefined4 *)((int)this + 0x390) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(undefined4 *)((int)this + 0x3a4) = 0;
  *(uint *)((int)this + 0x150) = (uint)(in_stack_00000038 != 0);
  *(float *)((int)this + 8) = DAT_0066c470;
  *(float *)((int)this + 0xc) = DAT_0066c470;
  if (*(int *)((int)this + 0x13c) == 0x15) {
    *(undefined4 *)((int)this + 0xc) = 0x41980000;
  }
  else {
    *(float *)((int)this + 0xc) = DAT_0066c470 * _DAT_0066c4a4;
  }
  fVar3 = *(float *)((int)this + 0xc) * _DAT_0066c4a4;
  *(undefined4 *)((int)this + 0x400) = in_stack_000000a8;
  *(float *)((int)this + 0x160) = fVar3;
  fVar3 = _DAT_0066c458 * _DAT_0066c4a8;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0xa8) = 0xbf800000;
  *(float *)((int)this + 0x9c) = fVar3;
  iVar4 = FUN_00597530();
  if ((*(int *)(iVar4 + 0x668) == 0) && (fVar8 = FUN_005b0be0(this), (float10)_DAT_0066c488 < fVar8)
     ) {
    in_stack_00000048 = (undefined4 *)(_DAT_0066c458 * _DAT_0066c4ac);
    fVar8 = FUN_005b0be0(this);
    fVar8 = fVar8 * (float10)(float)in_stack_00000048 * (float10)_DAT_0066c4b0;
    if ((*(int *)((int)this + 0x13c) == 0x15) &&
       (in_stack_00000048 = (undefined4 *)((float)in_stack_00000048 * _DAT_0066c4b4),
       (float10)(float)in_stack_00000048 <= fVar8)) {
      fVar8 = (float10)(float)in_stack_00000048;
    }
    *(float *)((int)this + 0x9c) = (float)fVar8;
    *(undefined4 *)((int)this + 0xa0) = 0;
    *(undefined4 *)((int)this + 0xa4) = 0;
    *(undefined4 *)((int)this + 0xa8) = 0xbf800000;
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x00000014);
  ExceptionList = local_c;
  return this;
}



void * __thiscall
FUN_00606380(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,int param_6,undefined4 param_7,undefined4 param_8,int param_9,
            undefined4 param_10,uint param_11,undefined4 param_12,undefined4 param_13,
            undefined4 param_14)

{
  undefined4 *puVar1;
  bool bVar2;
  HGDIOBJ pvVar3;
  void *this_00;
  int *piVar4;
  undefined3 extraout_var;
  undefined2 extraout_var_00;
  undefined4 uVar5;
  int iVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined1 uVar9;
  uint uVar10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656561;
  pvStack_c = ExceptionList;
  uVar10 = 0;
  uVar8 = 0;
  uVar7 = 0;
  iVar6 = 0;
  uVar5 = 0;
  ExceptionList = &pvStack_c;
  pvVar3 = GetStockObject(5);
  FUN_005ff910(this,param_1,s_CoolGameWindow_006ac804,param_11,param_2,param_3,param_4,param_5,
               param_6,(char *)0x0,pvVar3,uVar5,iVar6,uVar7,uVar8,uVar10);
  uStack_4 = 0;
  FUN_0060f420((undefined4 *)((int)this + 0xf8));
  *(undefined4 *)((int)this + 0x94) = param_7;
  uStack_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_0066e060;
  *(undefined4 *)((int)this + 0x98) = param_8;
  *(int *)((int)this + 0x9c) = param_9;
  if (param_9 != 0) {
    this_00 = (void *)FUN_00618b70(0x28);
    uStack_4._0_1_ = 2;
    if (this_00 == (void *)0x0) {
      piVar4 = (int *)0x0;
    }
    else {
      uVar10 = CONCAT22(extraout_var_00,*(undefined2 *)((int)this + 0x9c));
      uVar9 = 2;
      uVar5 = FUN_006037f0(*(int *)((int)this + 0x98));
      piVar4 = (int *)FUN_005fbd20(this_00,uVar5,uVar10,uVar9);
    }
    uStack_4._0_1_ = 1;
    *(int **)((int)this + 0xa0) = piVar4;
    bVar2 = FUN_005fc040(piVar4);
    if (CONCAT31(extraout_var,bVar2) != 0) goto LAB_00606490;
    puVar1 = *(undefined4 **)((int)this + 0xa0);
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
  }
  *(undefined4 *)((int)this + 0xa0) = 0;
LAB_00606490:
  *(undefined4 *)((int)this + 0xb8) = param_12;
  *(undefined4 *)((int)this + 0x24) = param_10;
  *(undefined4 *)((int)this + 0xac) = param_10;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0x104) = 0;
  *(undefined4 *)((int)this + 0x108) = 0;
  *(undefined4 *)((int)this + 0xa8) = 0;
  *(undefined4 *)((int)this + 0xb0) = param_13;
  *(undefined4 *)((int)this + 0xb4) = param_14;
  *(undefined4 *)((int)this + 0xd0) = 0;
  *(undefined4 *)((int)this + 0xd4) = 0;
  *(undefined4 *)((int)this + 0xd8) = 0;
  *(undefined4 *)((int)this + 0xdc) = 0;
  *(undefined4 *)((int)this + 0xe0) = 0;
  SetRectEmpty((LPRECT)((int)this + 0xbc));
  if (param_6 == 0) {
    *(undefined4 *)((int)this + 0xcc) = 0;
  }
  else {
    *(undefined4 *)((int)this + 0xcc) = *(undefined4 *)(param_6 + 0x18);
  }
  *(undefined4 *)((int)this + 0xf4) = 1;
  *(undefined4 *)((int)this + 0x10c) = 0;
  if ((param_11 & 0x10000000) == 0x10000000) {
    *(undefined4 *)((int)this + 0x110) = 1;
  }
  else {
    *(undefined4 *)((int)this + 0x110) = 0;
  }
  ExceptionList = pvStack_c;
  return this;
}


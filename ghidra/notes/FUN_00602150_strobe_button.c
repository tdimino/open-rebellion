
void * __thiscall
FUN_00602150(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,uint param_8,uint param_9,
            uint *param_10,int param_11,int param_12)

{
  HGDIOBJ pvVar1;
  void *pvVar2;
  int iVar3;
  undefined4 uVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  uint uVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656217;
  pvStack_c = ExceptionList;
  uVar8 = 0;
  uVar7 = 0;
  uVar6 = 0;
  iVar5 = 0;
  uVar4 = 0;
  ExceptionList = &pvStack_c;
  pvVar1 = GetStockObject(4);
  FUN_005ff910(this,param_1,s_CoolStrobeButton_006ac7d8,0x46000000,param_2,param_3,param_4,param_5,
               param_6,(char *)0x0,pvVar1,uVar4,iVar5,uVar6,uVar7,uVar8);
  uStack_4 = 0;
  FUN_00610b20((void *)((int)this + 0xe0),0,0,0,0,0,0);
  uStack_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_0066df58;
  *(undefined4 *)((int)this + 0x24) = param_7;
  *(undefined4 *)((int)this + 0xc4) = 0x10020;
  if ((short)param_8 == 0) {
    *(undefined4 *)((int)this + 0x94) = 0;
  }
  else {
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4._0_1_ = 2;
    if (pvVar2 == (void *)0x0) {
      *(undefined4 *)((int)this + 0x94) = 0;
    }
    else {
      pvVar2 = FUN_005fbd20(pvVar2,param_1,param_8,10);
      *(void **)((int)this + 0x94) = pvVar2;
    }
  }
  uStack_4._0_1_ = 1;
  if ((short)param_9 == 0) {
    *(undefined4 *)((int)this + 0x98) = 0;
  }
  else {
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4._0_1_ = 3;
    if (pvVar2 == (void *)0x0) {
      *(undefined4 *)((int)this + 0x98) = 0;
    }
    else {
      pvVar2 = FUN_005fbd20(pvVar2,param_1,param_9,10);
      *(void **)((int)this + 0x98) = pvVar2;
    }
  }
  uStack_4._0_1_ = 1;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 0x9c) = 0;
  *(short *)((int)this + 0xac) = (short)param_8;
  *(short *)((int)this + 0xae) = (short)param_9;
  *(undefined2 *)((int)this + 0xb0) = 0;
  *(undefined2 *)((int)this + 0xb2) = 0;
  *(undefined4 *)((int)this + 200) = 0;
  *(undefined4 *)((int)this + 0xb4) = 0x2000000;
  *(undefined4 *)((int)this + 0xb8) = 0x2ffffff;
  *(undefined4 *)((int)this + 0xc0) = 0x2404040;
  *(undefined4 *)((int)this + 0xbc) = 0x2008000;
  *(undefined2 *)((int)this + 0xdc) = 0;
  *(uint **)((int)this + 0xa4) = param_10;
  *(undefined4 *)((int)this + 0x10c) = param_2;
  *(undefined4 *)((int)this + 0x110) = param_3;
  pvVar2 = (void *)FUN_00618b70(0x28);
  uStack_4._0_1_ = 4;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_005fbda0(pvVar2,*(int *)((int)this + 0x30),*(int *)((int)this + 0x34),param_10);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  *(void **)((int)this + 0xa8) = pvVar2;
  if (*(int **)((int)this + 0x94) == (int *)0x0) {
LAB_00602359:
    if (*(int **)((int)this + 0x98) == (int *)0x0) goto LAB_0060238b;
    iVar5 = *(int *)((int)this + 0x30);
    iVar3 = FUN_005fc0e0(*(int **)((int)this + 0x98));
    if (iVar3 == iVar5) {
      iVar5 = *(int *)((int)this + 0x34);
      iVar3 = FUN_005fc0f0(*(int **)((int)this + 0x98));
      if (iVar3 == iVar5) goto LAB_0060238b;
    }
  }
  else {
    iVar5 = *(int *)((int)this + 0x30);
    iVar3 = FUN_005fc0e0(*(int **)((int)this + 0x94));
    if (iVar3 == iVar5) {
      iVar5 = *(int *)((int)this + 0x34);
      iVar3 = FUN_005fc0f0(*(int **)((int)this + 0x94));
      if (iVar3 == iVar5) goto LAB_00602359;
    }
  }
  *(uint *)((int)this + 0xc4) = *(uint *)((int)this + 0xc4) | 0x20000;
LAB_0060238b:
  if (param_12 == 0) {
    *(uint *)((int)this + 0xc4) = *(uint *)((int)this + 0xc4) | 0x10;
  }
  *(undefined4 *)((int)this + 0x100) = 0;
  *(int *)((int)this + 0xfc) = param_11;
  if (param_11 < 1) {
    *(undefined4 *)((int)this + 0x104) = 0;
  }
  else {
    *(int *)((int)this + 0x104) = (int)(0xfa / (longlong)param_11);
  }
  ExceptionList = pvStack_c;
  return this;
}


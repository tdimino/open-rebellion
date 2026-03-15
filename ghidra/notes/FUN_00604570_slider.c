
void * __thiscall
FUN_00604570(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5,undefined4 param_6,uint *param_7,uint param_8,uint param_9,
            undefined4 param_10,undefined4 param_11)

{
  HGDIOBJ pvVar1;
  void *pvVar2;
  int *piVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  uint uVar9;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656459;
  pvStack_c = ExceptionList;
  uVar9 = 0;
  uVar8 = 0;
  uVar7 = 0;
  iVar6 = 0;
  uVar5 = 0;
  ExceptionList = &pvStack_c;
  pvVar1 = GetStockObject(4);
  FUN_005ff910(this,param_1,s_slider_006ac7ec,0x54000000,param_2,param_3,0,0,param_4,(char *)0x0,
               pvVar1,uVar5,iVar6,uVar7,uVar8,uVar9);
  *(undefined4 *)((int)this + 0x98) = param_3;
  *(undefined4 *)((int)this + 0x24) = param_5;
  *(undefined4 *)((int)this + 0xa4) = param_6;
  uStack_4._0_1_ = 0;
  uStack_4._1_3_ = 0;
  *(undefined ***)this = &PTR_FUN_0066e010;
  *(undefined4 *)((int)this + 0xb4) = 0;
  *(uint **)((int)this + 0xb0) = param_7;
  *(undefined4 *)((int)this + 0x94) = param_2;
  *(undefined4 *)((int)this + 0xdc) = param_10;
  *(undefined4 *)((int)this + 0xe0) = param_11;
  *(undefined4 *)((int)this + 200) = 0;
  *(undefined4 *)((int)this + 0xcc) = 0;
  *(undefined4 *)((int)this + 0xd4) = 0;
  *(undefined4 *)((int)this + 0xd8) = 0;
  if (param_8 == 0) {
    *(undefined4 *)((int)this + 0xa8) = 0;
  }
  else {
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4._0_1_ = 1;
    if (pvVar2 == (void *)0x0) {
      *(undefined4 *)((int)this + 0xa8) = 0;
    }
    else {
      pvVar2 = FUN_005fbd20(pvVar2,param_1,param_8,2);
      *(void **)((int)this + 0xa8) = pvVar2;
    }
  }
  uStack_4._0_1_ = 0;
  if (param_9 == 0) {
    *(undefined4 *)((int)this + 0xac) = 0;
  }
  else {
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4._0_1_ = 2;
    if (pvVar2 == (void *)0x0) {
      piVar3 = (int *)0x0;
    }
    else {
      piVar3 = (int *)FUN_005fbd20(pvVar2,param_1,param_9,2);
    }
    uStack_4._0_1_ = 0;
    *(int **)((int)this + 0xac) = piVar3;
    if (piVar3 != (int *)0x0) {
      iVar6 = FUN_005fc0e0(piVar3);
      *(int *)((int)this + 0x9c) = iVar6;
      iVar6 = FUN_005fc0f0(*(int **)((int)this + 0xac));
      *(int *)((int)this + 0xa0) = iVar6;
    }
  }
  if (*(int **)((int)this + 0xa8) != (int *)0x0) {
    iVar6 = FUN_005fc0e0(*(int **)((int)this + 0xa8));
    iVar4 = FUN_005fc0f0(*(int **)((int)this + 0xa8));
    pvVar2 = (void *)FUN_00618b70(0x28);
    uStack_4 = CONCAT31(uStack_4._1_3_,3);
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005fbda0(pvVar2,iVar6,iVar4,param_7);
    }
    *(void **)((int)this + 0xb4) = pvVar2;
    *(undefined4 *)((int)this + 0xb8) = 0;
    *(undefined4 *)((int)this + 0xbc) = 0;
    *(int *)((int)this + 0x30) = iVar6;
    *(int *)((int)this + 0x34) = iVar4;
    *(int *)((int)this + 0xc0) = iVar6;
    *(int *)((int)this + 0xc4) = iVar4;
    *(int *)((int)this + 200) = iVar6 / 2;
    *(int *)((int)this + 0xcc) = iVar4 / 2;
    if (*(int *)((int)this + 0xa4) == 1) {
      *(int *)((int)this + 0xd8) = iVar4;
      *(uint *)((int)this + 0xcc) =
           (((uint)(iVar4 * 100) / *(uint *)((int)this + 0xe0)) * *(int *)((int)this + 0xdc)) / 100;
      *(int *)((int)this + 0xd0) = iVar6 / 2 - *(int *)((int)this + 0x9c) / 2;
    }
    else if (*(int *)((int)this + 0xa4) == 2) {
      *(undefined4 *)((int)this + 0xd4) = 0;
      *(uint *)((int)this + 200) =
           (((uint)(iVar6 * 100) / *(uint *)((int)this + 0xe0)) * *(int *)((int)this + 0xdc)) / 100;
      *(int *)((int)this + 0xd0) = iVar4 / 2 - *(int *)((int)this + 0xa0) / 2;
    }
  }
  ExceptionList = pvStack_c;
  return this;
}



void * __thiscall
FUN_00604cf0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,int param_6,uint *param_7,uint param_8)

{
  HGDIOBJ pvVar1;
  HDC hDC;
  void *pvVar2;
  undefined4 uVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  uint uVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006564bf;
  pvStack_c = ExceptionList;
  uVar7 = 0;
  uVar6 = 0;
  uVar5 = 0;
  iVar4 = 0;
  uVar3 = 0;
  ExceptionList = &pvStack_c;
  pvVar1 = GetStockObject(5);
  FUN_005ff910(this,param_1,s_CoolStringField_006ac7f4,param_8 | 0x56000000,param_2,param_3,param_4,
               param_5,param_6,(char *)0x0,pvVar1,uVar3,iVar4,uVar5,uVar6,uVar7);
  uStack_4 = 0;
  FUN_005f4950((undefined4 *)((int)this + 0x94),0);
  uStack_4._0_1_ = 1;
  FUN_005f2f50((undefined4 *)((int)this + 0x98));
  *(undefined ***)this = &PTR_FUN_0066e038;
  *(undefined4 *)((int)this + 0x94) = 1;
  *(uint **)((int)this + 0xa8) = param_7;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0xc4) = param_2;
  *(undefined4 *)((int)this + 200) = param_3;
  uStack_4._0_1_ = 2;
  hDC = GetDC(*(HWND *)(param_6 + 0x18));
  *(undefined4 *)((int)this + 0xe0) = 1;
  *(int *)((int)this + 0xd8) = param_4;
  *(int *)((int)this + 0xdc) = param_5;
  pvVar2 = (void *)FUN_00618b70(0x28);
  uStack_4._0_1_ = 3;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_005fbea0(pvVar2,(*(int *)((int)this + 0xe0) + param_4) * 2,
                          (param_5 + *(int *)((int)this + 0xe0)) * 2,param_7,hDC);
  }
  *(void **)((int)this + 0xac) = pvVar2;
  *(undefined4 *)((int)this + 0xb0) = 0;
  uStack_4 = CONCAT31(uStack_4._1_3_,2);
  ReleaseDC(*(HWND *)(param_6 + 0x18),hDC);
  *(undefined4 *)((int)this + 0xe8) = 0;
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(int *)((int)this + 0xc0) = param_4;
  *(undefined4 *)((int)this + 0xd0) = 0;
  *(undefined4 *)((int)this + 0xd4) = 0;
  *(undefined4 *)((int)this + 0xe4) = 0xffffff;
  *(undefined4 *)((int)this + 0xec) = 0xff;
  *(undefined4 *)((int)this + 0xf0) = 0x808080;
  *(undefined4 *)((int)this + 0xe0) = 1;
  *(undefined4 *)((int)this + 0xb4) = 0xffffffff;
  *(undefined4 *)((int)this + 0xb8) = 0xffffffff;
  *(undefined4 *)((int)this + 0xcc) = 0xffffffff;
  ExceptionList = pvStack_c;
  return this;
}


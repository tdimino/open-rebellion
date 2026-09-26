
void * __thiscall
FUN_0060e150(void *this,undefined4 param_1,undefined4 param_2,int param_3,int param_4,int param_5,
            COLORREF param_6,COLORREF param_7,uint param_8)

{
  undefined4 uVar1;
  HGDIOBJ pvVar2;
  undefined4 uVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  uint uVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656c68;
  pvStack_c = ExceptionList;
  uVar1 = *(undefined4 *)(param_5 + 0x1c);
  uVar7 = 0;
  uVar6 = 0;
  uVar5 = 0;
  iVar4 = 0;
  uVar3 = 0;
  ExceptionList = &pvStack_c;
  pvVar2 = GetStockObject(4);
  FUN_005ff910(this,uVar1,s_CoolwinStatusBar_006ac940,param_8,param_1,param_2,param_3,param_4,
               param_5,(char *)0x0,pvVar2,uVar3,iVar4,uVar5,uVar6,uVar7);
  uStack_4 = 0;
  *(COLORREF *)((int)this + 0xa4) = param_7 + 1;
  *(undefined ***)this = &PTR_FUN_0066e258;
  *(undefined4 *)((int)this + 0x94) = 100;
  *(undefined4 *)((int)this + 0x9c) = 0;
  *(undefined4 *)((int)this + 0x98) = 0;
  *(COLORREF *)((int)this + 0xa0) = param_6 + 1;
  *(undefined4 *)((int)this + 0xac) = 0;
  *(undefined4 *)((int)this + 0xb0) = 0;
  FUN_0060e2d0(this,param_6);
  FUN_0060e340(this,param_7);
  *(undefined4 *)((int)this + 0xb4) = 0;
  ExceptionList = pvStack_c;
  return this;
}


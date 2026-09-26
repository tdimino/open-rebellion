
void * __thiscall
FUN_005ff1c0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,uint param_8)

{
  HGDIOBJ pvVar1;
  undefined4 uVar2;
  int iVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  uint uVar6;
  
  uVar6 = 0;
  uVar5 = 0;
  uVar4 = 0;
  iVar3 = 0;
  uVar2 = 0;
  pvVar1 = GetStockObject(4);
  FUN_005ff910(this,param_1,s_GAME_WINDOW_OBJECT_006ac7a4,param_8 | 0x46000000,param_2,param_3,
               param_4,param_5,param_6,(char *)0x0,pvVar1,uVar2,iVar3,uVar4,uVar5,uVar6);
  *(undefined ***)this = &PTR_FUN_0066ded8;
  *(undefined4 *)((int)this + 0x94) = param_7;
  *(undefined4 *)((int)this + 0x98) = 0;
  return this;
}


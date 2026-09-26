
void * __thiscall
FUN_0060aa70(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,undefined2 param_8,undefined2 param_9,
            undefined4 param_10,undefined4 param_11,undefined4 param_12,undefined4 param_13)

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
  FUN_005ff910(this,param_1,s_Scroll_Bar_006ac90c,0x56000000,param_2,param_3,param_4,param_5,param_6
               ,(char *)0x0,pvVar1,uVar2,iVar3,uVar4,uVar5,uVar6);
  *(undefined4 *)((int)this + 0x98) = param_7;
  *(undefined2 *)((int)this + 0xe8) = param_8;
  *(undefined4 *)((int)this + 0xe0) = param_11;
  *(undefined2 *)((int)this + 0xea) = param_9;
  *(undefined4 *)((int)this + 0xec) = param_13;
  *(undefined4 *)((int)this + 0xb4) = 0;
  *(undefined4 *)((int)this + 0xb8) = 0;
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(undefined4 *)((int)this + 0xc0) = 0;
  *(undefined4 *)((int)this + 0xc4) = 0;
  *(undefined4 *)((int)this + 200) = 0;
  *(undefined4 *)((int)this + 0xcc) = 0;
  *(undefined4 *)((int)this + 0xd0) = 0;
  *(undefined4 *)((int)this + 0xd4) = 0;
  *(undefined4 *)((int)this + 0xd8) = 0;
  *(undefined4 *)((int)this + 0x9c) = 0;
  *(undefined4 *)((int)this + 0xb0) = 0;
  *(undefined4 *)((int)this + 0x94) = 0;
  *(undefined ***)this = &PTR_FUN_0066e158;
  *(undefined4 *)((int)this + 0xe4) = param_12;
  *(undefined4 *)((int)this + 0xdc) = param_10;
  return this;
}


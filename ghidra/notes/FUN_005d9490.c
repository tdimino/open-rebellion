
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall
FUN_005d9490(void *this,float param_1,undefined4 param_2,int *param_3,int *param_4,
            undefined4 param_5,int param_6,int param_7,int param_8,int param_9)

{
  float fVar1;
  int *piVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654b08;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005d9eb0(this,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9);
  local_4 = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x40) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x30) = 5;
  *(undefined4 *)((int)this + 0x34) = 5;
  piVar2 = *(int **)((int)this + 4);
  *(undefined ***)this = &PTR_FUN_0066d418;
  fVar1 = (float)_DAT_0066d400;
  *(float *)((int)this + 0x4c) = param_1;
  *(undefined4 *)((int)this + 0x48) = 0x3f800000;
  (**(code **)(*piVar2 + 0x3c))(piVar2,param_1 * fVar1);
  (**(code **)(**(int **)((int)this + 4) + 0x38))(*(int **)((int)this + 4),0x3f800000);
  ExceptionList = piVar2;
  return this;
}



void * __thiscall FUN_005d9a70(void *this,undefined4 param_1,undefined4 param_2,int param_3)

{
  void *unaff_ESI;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654b28;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005da030(this,param_3);
  *(undefined ***)this = &PTR_FUN_0066d478;
  *(undefined4 *)((int)this + 0x34) = param_1;
  *(undefined4 *)((int)this + 0x30) = param_2;
  *(undefined4 *)((int)this + 0x38) = 0x3ca3d70a;
  local_4 = 0;
  (**(code **)(**(int **)((int)this + 4) + 0x38))(*(int **)((int)this + 4),0x3dcccccd);
  ExceptionList = unaff_ESI;
  return this;
}


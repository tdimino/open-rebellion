
bool FUN_0055fe70(uint *param_1,void *param_2,void *param_3)

{
  void *pvVar1;
  int *this;
  uint *puVar2;
  int iVar3;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a0f8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (int *)FUN_00505190(param_1);
  bVar4 = this != (int *)0x0;
  if (this != (int *)0x0) {
    if (((this[0x14] & 2U) == 0) && (bVar4)) {
      bVar4 = true;
    }
    else {
      bVar4 = false;
    }
    if ((this[0x14] & 2U) == 0) {
      puVar2 = FUN_004025b0(param_2,(uint *)&param_1);
      pvVar1 = param_3;
      local_4 = 0;
      iVar3 = (**(code **)(*this + 0xa8))(puVar2,param_3);
      if ((iVar3 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      iVar3 = FUN_004f7480(this,1,pvVar1);
      if ((iVar3 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
      iVar3 = FUN_004f74f0(this,1,pvVar1);
      if ((iVar3 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
  }
  ExceptionList = local_c;
  return bVar4;
}


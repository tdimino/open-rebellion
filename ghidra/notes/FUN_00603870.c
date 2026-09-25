
void * __thiscall
FUN_00603870(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,int param_8,uint param_9,uint param_10
            ,uint *param_11,int param_12,int param_13)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656306;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00602150(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,0,param_9,param_11,
               param_12,param_13);
  local_4 = 0;
  FUN_0060f2d0((undefined4 *)((int)this + 0x114));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0066df88;
  FUN_00603aa0(this,param_8,param_10 & 0xffff);
  ExceptionList = local_c;
  return this;
}

